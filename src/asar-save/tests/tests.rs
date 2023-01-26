use asar_save::{AsarArchive, AsarFile, AsarHeader, Game, Metadata};
use std::io::{Cursor, Write};

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

    let files = vec![
        (
            String::from("meta"),
            AsarFile {
                offset: 0,
                size: 21,
            },
        ),
        (
            String::from("gamestate"),
            AsarFile {
                offset: 21,
                size: 22,
            },
        ),
        (
            String::from("ai"),
            AsarFile {
                offset: 43,
                size: 18,
            },
        ),
    ];

    let meta = AsarHeader {
        files,
        game: Game::Eu4,
        filename: String::from("test.eu4"),
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
