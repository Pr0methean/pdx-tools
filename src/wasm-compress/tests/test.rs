use std::io::{Cursor, Read};

#[test]
fn test_recompression_plaintext() {
    let data = b"hello world";
    let compressed = wasm_compress::compress(&data[..]).unwrap();

    let reader = Cursor::new(compressed);
    let mut decoder = flate2::read::GzDecoder::new(reader);
    let mut out = Vec::new();
    decoder.read_to_end(&mut out).unwrap();
    assert_eq!(out.as_slice(), &data[..]);
}

#[test]
fn test_recompression_zip() {
    let data = include_bytes!("test.zip");
    let compressed = wasm_compress::compress(&data[..]).unwrap();
    assert_eq!(&data[..], compressed.as_slice());
}
