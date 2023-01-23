import { transfer } from "comlink";
import { debugLog } from "@/lib/debug";
import { CompressionPayload, CompressProgressCb } from "./compress-types";

interface WasmCompressor {
  compress(data: Uint8Array, cb: CompressProgressCb): Uint8Array;
  http_upload_headers(data: Uint8Array): string;
}

export function compress_data(
  compressor: WasmCompressor,
  data: Uint8Array,
  cb: CompressProgressCb
): CompressionPayload {
  const start = performance.now();
  const startKb = (data.length / 1024).toFixed(0);
  const result = compressor.compress(data, cb);
  const end = performance.now();
  const endKb = (result.length / 1024).toFixed(0);
  debugLog(
    `compressed: ${startKb}KB to ${endKb}KB in ${(end - start).toFixed(2)}ms`
  );

  const meta = JSON.parse(compressor.http_upload_headers(data));
  return transfer(
    {
      contentType: meta.content_type,
      ...(meta.content_encoding && {
        contentEncoding: meta.content_encoding,
      }),
      data: result,
    },
    [result.buffer]
  );
}
