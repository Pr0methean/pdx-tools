use crate::{AsarError, AsarFile, AsarHeader, Game, Metadata};
use flate2::read::DeflateDecoder;
use std::io::{self, Cursor, Read, Write};

#[derive(Debug)]
struct ZipEntry {
    zip_pos: usize,
    order: usize,
    size: u64,
}

fn new_brotli<W: Write>(writer: W) -> brotli::CompressorWriter<W> {
    brotli::CompressorWriter::new(writer, 4096, 9, 22)
}

fn generate_header(header: &AsarHeader) -> Vec<u8> {
    let mut head = vec![0u8; 2048];
    head.truncate(16);
    serde_json::to_writer(&mut head, &header).unwrap();
    let json_size = head.len() - 16;
    let padding = (4 - (json_size % 4)) % 4;
    let header_size = (json_size + padding) as usize;
    head[..4].copy_from_slice(&4u32.to_le_bytes());
    head[4..8].copy_from_slice(&((header_size as u32) + 8).to_le_bytes());
    head[8..12].copy_from_slice(&((header_size as u32) + 4).to_le_bytes());
    head[12..16].copy_from_slice(&(json_size as u32).to_le_bytes());
    head.resize(header_size + 16, 0);
    head
}

pub fn create_asar(input: &[u8], game: Game, meta: &Metadata) -> Result<Vec<u8>, AsarError> {
    let reader = Cursor::new(input);
    if let Ok(mut zip) = zip::ZipArchive::new(reader) {
        let mut original_header_line = None;
        let mut new_header_line = String::new();
        if game.has_header_line() {
            let first_line_pos = input[..zip.offset() as usize]
                .iter()
                .position(|&x| x == b'\n')
                .ok_or(AsarError::HeaderLine)?;

            let raw_line = &input[..first_line_pos + 1];

            if raw_line.len() < 24 {
                return Err(AsarError::HeaderLine);
            }

            let header_line = std::str::from_utf8(raw_line)?;
            original_header_line = Some(String::from(header_line.trim_end()));
            let mut raw_new = raw_line.to_vec();

            // simple way of checking if an ascii digit is odd
            if raw_new[6] & 1 == 0 {
                raw_new[6] = b'0' // uncompressed plaintext
            } else {
                raw_new[6] = b'1' // uncompressed binary
            };

            new_header_line = String::from_utf8(raw_new)
                .map_err(|x| AsarError::HeaderEncoding(x.utf8_error()))?;
        }

        let zip_order = game.zip_order();
        let mut zip_files = Vec::with_capacity(zip_order.len());
        for i in 0..zip.len() {
            let file = zip.by_index_raw(i)?;
            let order = zip_order.iter().position(|&x| x == file.name());
            let order = match order {
                Some(x) => x,
                _ => continue,
            };

            let header = game.zip_file_header();

            let size = if order == 0 {
                file.size()
            } else {
                file.size() - (header.len() as u64)
            };

            zip_files.push(ZipEntry {
                zip_pos: i,
                order,
                size,
            });
        }

        zip_files.sort_unstable_by_key(|x| x.order);
        let mut zip_entries = Vec::with_capacity(zip_files.len());
        let mut offset = 0;
        for entry in &zip_files {
            zip_entries.push((
                String::from(zip_order[entry.order]),
                AsarFile {
                    offset,
                    size: entry.size,
                },
            ));

            offset += entry.size;
        }

        let header = AsarHeader {
            files: zip_entries,
            filename: String::from(meta.filename),
            game,
            original_header_line,
        };

        let head = generate_header(&header);
        let out = Vec::with_capacity(input.len() / 2);
        let mut compressor = new_brotli(out);
        compressor
            .write_all(&head)
            .map_err(AsarError::Compression)?;

        let mut scratch = vec![0u8; game.zip_file_header().len()];
        for entry in zip_files {
            let file = zip.by_index_raw(entry.zip_pos)?;
            let start = file.data_start() as usize;
            let mut inflater =
                DeflateDecoder::new(&input[start..start + file.compressed_size() as usize]);

            if entry.order != 0 {
                inflater
                    .read_exact(&mut scratch)
                    .map_err(|x| AsarError::ZipBadData { msg: x.to_string() })?;
            } else if !new_header_line.is_empty() {
                let mut line_reader = new_header_line.as_bytes();
                io::copy(&mut line_reader, &mut compressor)
                    .map_err(|x| AsarError::ZipBadData { msg: x.to_string() })?;
            }

            io::copy(&mut inflater, &mut compressor)
                .map_err(|x| AsarError::ZipBadData { msg: x.to_string() })?;
        }

        Ok(compressor.into_inner())
    } else {
        let entries = vec![(
            String::from(meta.filename),
            AsarFile {
                offset: 0,
                size: input.len() as u64,
            },
        )];

        let header = AsarHeader {
            files: entries,
            filename: String::from(meta.filename),
            game,
            ..Default::default()
        };

        let head = generate_header(&header);
        let out = Vec::with_capacity(input.len() / 10);
        let mut compressor = new_brotli(out);
        compressor
            .write_all(&head)
            .map_err(AsarError::Compression)?;
        compressor
            .write_all(input)
            .map_err(AsarError::Compression)?;
        Ok(compressor.into_inner())
    }
}
