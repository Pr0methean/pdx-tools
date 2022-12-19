use flate2::Compression;
use wasm_bindgen::JsValue;
use std::io::{Cursor, Read};

struct ProgressReader<'a, R> {
    reader: R,
    current_size: usize,
    total_size: usize,
    progress: Option<&'a js_sys::Function>,
    read_cycle: usize,
}

impl<'a, R> ProgressReader<'a, R> {
    pub fn new(reader: R, total_size: usize, progress: Option<&'a js_sys::Function>) -> Self {
        Self {
            reader,
            total_size,
            progress,
            current_size: 0,
            read_cycle: 0,
        }
    }
}

impl<'a, R> Read for ProgressReader<'a, R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let read = self.reader.read(buf)?;
        self.read_cycle += 1;
        self.current_size += read;
        if self.read_cycle % 100 == 0 {
            if let Some(cb) = self.progress {
                let progress = (self.current_size as f64) / (self.total_size as f64);
                let this = JsValue::null();
                let arg = JsValue::from_f64(progress.min(1.0));
                let _ = cb.call1(&this, &arg);
            }
        }

        Ok(read)
    }
}

fn _recompress<'a>(
    data: &[u8],
    f: Option<&js_sys::Function>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let reader = Cursor::new(data);
    if zip::ZipArchive::new(reader).is_ok() {
        Ok(data.to_vec())
    } else {
        let inner = Cursor::new(data);
        let mut reader = ProgressReader::new(inner, data.len(), f);
        let out = Vec::with_capacity(data.len() / 10);
        let cursor = Cursor::new(out);
        let mut compressor = flate2::write::GzEncoder::new(cursor, Compression::default());
        std::io::copy(&mut reader, &mut compressor)?;
        let data = compressor.finish()?.into_inner();
        Ok(data)
    }
}

pub fn compress(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    _recompress(data, None)
}

pub fn compress_cb(data: &[u8], f: &js_sys::Function) -> Result<Vec<u8>, JsValue> {
    _recompress(data, Some(f)).map_err(|e| JsValue::from_str(e.to_string().as_str()))
}

pub fn http_upload_headers(data: &[u8]) -> String {
    let reader = Cursor::new(data);
    if zip::ZipArchive::new(reader).is_ok() {
        String::from(r#"{"content_type":"application/zip"}"#)
    } else {
        String::from(r#"{"content_type":"text/plain", "content_encoding": "gzip"}"#)
    }
}
