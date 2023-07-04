import { timeSync } from "@/lib/timeit";
import { AchievementsScore } from "../types/models";
import { wasm } from "./common";
import * as mod from "../../../../../wasm-eu4/pkg/wasm_eu4";
import { fetchOk } from "@/lib/fetch";
import { type Eu4SaveInput } from "../store";
import { logMs } from "@/lib/log";
import { captureException } from "@/features/errors";

export const initializeWasm = wasm.initializeModule;
export async function fetchData(save: Eu4SaveInput) {
  console.log("HELO");
  switch (save.kind) {
    case "handle": {
      const file = await save.file.getFile();
      const lastModified = file.lastModified;
      const data = await file.arrayBuffer();
      wasm.stash(new Uint8Array(data), {
        kind: "handle",
        file: save.file,
        lastModified,
      });
      return;
    }
    case "file": {
      const data = await save.file.arrayBuffer();
      wasm.stash(new Uint8Array(data), { kind: "file", file: save.file });
      return;
    }
    case "server": {
      const url = `/api/saves/${save.saveId}/file`;
      const data = await fetchOk(url).then((x) => x.arrayBuffer());
      wasm.stash(new Uint8Array(data), { kind: "remote", url });
      return;
    }
    case "skanderbeg": {
      // Unable to stream this as the skanderbeg saves as the content-length does
      // not actually match the body as the body is compressed twice and the
      // reported length is only the first compression
      const url = `https://skanderbeg.pm/api.php?scope=downloadSaveFile&id=${save.skanId}`;
      const data = await fetchOk(url).then((x) => x.arrayBuffer());
      wasm.stash(new Uint8Array(data), { kind: "remote", url });
      return;
    }
  }
}

export async function eu4GameParse(
  gameData: Uint8Array,
  provinceIdToColorIndex: Uint16Array
) {
  const data = wasm.takeStash();
  wasm.save = wasm.module.parse_save(data, gameData, provinceIdToColorIndex);
  const meta = getMeta(wasm.save);
  const achievements = wasm.save.get_achievements();
  const defaultSelectedTag = eu4DefaultSelectedTag(meta);
  return { meta, achievements, defaultSelectedTag };
}

export async function parseMeta() {
  const meta = wasm.module.parse_meta(wasm.viewStash());
  const version = `${meta.savegame_version.first}.${meta.savegame_version.second}`;
  return { meta, version };
}

export type EnhancedMeta = ReturnType<typeof getMeta>;
export function getMeta(savefile: mod.SaveFile) {
  const meta = savefile.get_meta_raw();
  const starting_tag = savefile.get_starting_country() ?? null;
  const starting_tag_name = starting_tag
    ? savefile.localize_country(starting_tag)
    : null;

  const start_date = savefile.get_start_date();
  const total_days = savefile.get_total_days();
  const player_tag_name = savefile.localize_country(meta.player);
  const dlc = savefile.get_dlc_ids();
  const encoding = savefile.save_encoding();
  const players = savefile.get_players();
  const aliveCountries = savefile.get_alive_countries();
  const playthroughId = savefile.playthrough_id();
  const mode = savefile.save_mode();
  const gameplayOptions = savefile.gameplay_options();
  const warnings = savefile.savefile_warnings();

  return {
    start_date,
    total_days,
    starting_tag,
    starting_tag_name,
    player_tag_name,
    dlc,
    encoding,
    players,
    aliveCountries,
    playthroughId,
    mode,
    gameplayOptions,
    warnings,
    ...meta,
  };
}

function eu4DefaultSelectedTag(meta: EnhancedMeta): string {
  if (meta === undefined) {
    throw new Error("meta can't be undefined");
  }

  if (meta.player && meta.player !== "---") {
    return meta.player;
  }

  if (Object.keys(meta.players).length > 0) {
    return Object.keys(meta.players)[0];
  }

  if (meta.aliveCountries.length > 0) {
    return meta.aliveCountries[0];
  }

  throw new Error("unable to determine default selected country");
}

export function supportsFileObserver() {
  return wasm.supportsFileObserver();
}

let observer: ReturnType<(typeof wasm)["startFileObserver"]>;
export function startFileObserver<T>(
  frequency: string,
  cb: (save: { meta: EnhancedMeta; achievements: AchievementsScore }) => T
) {
  observer = wasm.startFileObserver(async (data) => {
    try {
      const reparse = timeSync(() => wasm.save.reparse(frequency, data));
      if (reparse.data.kind === "tooSoon") {
        logMs(reparse, `save date too soon to update: ${reparse.data.date}`);
        return;
      }

      logMs(reparse, "reparsed save");
      const achievements = wasm.save.get_achievements();
      cb({ meta: getMeta(wasm.save), achievements });
    } catch (ex) {
      captureException(ex, { tags: { msg: "file-watcher" } });
    }
  });
}

export function stopFileObserver() {
  observer.stopObserver();
}
