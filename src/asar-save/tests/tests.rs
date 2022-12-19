use std::{
    collections::HashMap,
    io::{Cursor, Write},
};

use asar_save::{AsarArchive, AsarDirectory, AsarEntry, AsarFile, AsarHeader, Game, Metadata};

struct ZipFile<'a> {
    name: &'a str,
    data: &'a [u8],
}

fn create_zip(files: &[ZipFile]) -> Vec<u8> {
    let out = Vec::new();
    let writer = Cursor::new(out);
    let mut out_zip = zip::ZipWriter::new(writer);
    let options =
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    for file in files {
        out_zip.start_file(file.name, options).unwrap();
        out_zip.write_all(file.data).unwrap();
    }
    out_zip.finish().unwrap().into_inner()
}

fn brotli_inflate(mut data: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    brotli::BrotliDecompress(&mut data, &mut out).unwrap();
    out
}

#[test]
fn test_eu4_txt_zip() {
    let zip = create_zip(&[
        ZipFile {
            name: "ai",
            data: b"EU4txt\nchecksum=\"abc123\"",
        },
        ZipFile {
            name: "gamestate",
            data: b"EU4txt\nstart_date=1444.11.11",
        },
        ZipFile {
            name: "meta",
            data: b"EU4txt\ndate=1817.8.31",
        },
    ]);

    let out = asar_save::create_asar(
        &zip,
        Game::Eu4,
        &Metadata {
            filename: "test.eu4",
        },
    )
    .unwrap();

    let asar_data = brotli_inflate(&out);
    let asar = AsarArchive::try_parse(&asar_data).unwrap();

    let mut files = HashMap::new();
    files.insert(
        String::from("meta"),
        AsarEntry::File(AsarFile {
            offset: 0,
            size: 21,
        }),
    );
    files.insert(
        String::from("gamestate"),
        AsarEntry::File(AsarFile {
            offset: 21,
            size: 22,
        }),
    );
    files.insert(
        String::from("ai"),
        AsarEntry::File(AsarFile {
            offset: 43,
            size: 18,
        }),
    );

    let mut dir_files = HashMap::new();
    dir_files.insert(
        String::from("test.eu4"),
        AsarEntry::Dir(AsarDirectory { files }),
    );

    let meta = AsarHeader {
        files: dir_files,
        game: Game::Eu4,
        ..Default::default()
    };

    assert_eq!(asar.header(), &meta);
    assert_eq!(
        asar.data(),
        b"EU4txt\ndate=1817.8.31\nstart_date=1444.11.11\nchecksum=\"abc123\""
    );
}

#[test]
fn test_eu4_txt_plain() {
    let out = asar_save::create_asar(
        b"EU4txt\ndate=1817.8.31\nstart_date=1444.11.11\nchecksum=\"abc123\"",
        Game::Eu4,
        &Metadata {
            filename: "test.eu4",
        },
    )
    .unwrap();

    let asar_data = brotli_inflate(&out);
    let asar = AsarArchive::try_parse(&asar_data).unwrap();
    assert_eq!(
        asar.data(),
        b"EU4txt\ndate=1817.8.31\nstart_date=1444.11.11\nchecksum=\"abc123\""
    );
}
