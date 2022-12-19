import { transfer, proxy } from "comlink";
import { useMemo } from "react";
import { getWasmWorker, useWasmWorker } from "../engine";
import { CompressProgressCb } from "./compress-types";

export const useCompression = () => {
  const worker = useWasmWorker();
  const ret = useMemo(
    () => ({
      compress: async (data: Uint8Array, cb: CompressProgressCb) => {
        return getWasmWorker(worker).compress(
          transfer(data, [data.buffer]),
          proxy(cb)
        );
      },
    }),
    [worker]
  );

  return ret;
};
