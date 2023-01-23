export interface ContentMetadata {
  contentEncoding: string;
  contentType: string;
}

export interface CompressionPayload extends ContentMetadata {
  data: Uint8Array;
}

export type CompressProgressCb = (portion: number) => void;
