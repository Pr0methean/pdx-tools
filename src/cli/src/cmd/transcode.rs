use crate::remote_parse::inflate_file;
use anyhow::Context;
use clap::Args;
use csv::Reader;
use serde::Deserialize;
use std::{
    collections::HashMap,
    io::{Cursor, Read, Write},
    path::PathBuf,
    process::ExitCode,
};
use walkdir::WalkDir;

/// Re-encode save container format
#[derive(Args)]
pub struct TranscodeArgs {
    /// Path to database export (csv)
    #[arg(long)]
    reference: Option<PathBuf>,

    #[arg(long)]
    dest: PathBuf,

    /// Files and directories to parse
    #[arg(action = clap::ArgAction::Append)]
    files: Vec<PathBuf>,
}

impl TranscodeArgs {
    pub fn run(&self) -> anyhow::Result<ExitCode> {
        let existing_records = if let Some(reference) = self.reference.as_ref() {
            let rdr = csv::Reader::from_path(&reference)
                .with_context(|| format!("unable to open: {}", reference.display()))?;
            extract_existing_records(rdr)?
        } else {
            HashMap::new()
        };

        let files = self
            .files
            .iter()
            .flat_map(|fp| WalkDir::new(fp).into_iter().filter_map(|e| e.ok()))
            .filter(|e| e.file_type().is_file());

        for file in files {
            let path = file.path();
            let save_id = String::from(path.file_name().unwrap().to_str().unwrap());
            let file = std::fs::File::open(path)
                .with_context(|| format!("unable to open: {}", path.display()))?;
            let inflated = inflate_file(&file)?;

            if let Ok(_) = asar_save::AsarArchive::try_parse(&inflated) {
                continue;
            }

            let data = if let Some(tar) = tarsave::extract_tarsave(&inflated) {
                let len = file.metadata().map_or(0, |x| x.len() / 5);
                let out = Vec::with_capacity(len as usize);
                let writer = Cursor::new(out);
                let mut out_zip = zip::ZipWriter::new(writer);
                let options = zip::write::FileOptions::default()
                    .compression_method(zip::CompressionMethod::Deflated);

                for (name, data) in &[
                    ("meta", tar.meta),
                    ("gamestate", tar.gamestate),
                    ("ai", tar.ai),
                ] {
                    out_zip.start_file(String::from(*name), options).unwrap();
                    out_zip.write_all(data).unwrap();
                }

                out_zip.finish().unwrap().into_inner()
            } else {
                inflated
            };

            let filename = existing_records
                .get(&save_id)
                .with_context(|| format!("unable to find save id: {}", path.display()))?;

            let out = asar_save::create_asar(
                &data[..],
                asar_save::Game::Eu4,
                &asar_save::Metadata {
                    filename: filename.as_str(),
                },
            )?;

            let out_path = self.dest.join(path.file_name().unwrap());
            std::fs::write(&out_path, &out)
                .with_context(|| format!("unable to write to {}", out_path.display()))?;
            println!("{} {}", out_path.display(), out.len());
        }

        Ok(ExitCode::SUCCESS)
    }
}

#[derive(Debug, Deserialize)]
struct FlatSave<'a> {
    id: &'a str,
    filename: &'a str,
}

fn extract_existing_records<T: Read>(
    mut rdr: Reader<T>,
) -> anyhow::Result<HashMap<String, String>> {
    let mut existing_records = HashMap::new();
    let mut raw_record = csv::StringRecord::new();
    let headers = rdr.headers().context("unable to get csv header")?.clone();
    while rdr.read_record(&mut raw_record)? {
        let record: FlatSave = raw_record.deserialize(Some(&headers))?;
        existing_records.insert(String::from(record.id), String::from(record.filename));
    }

    Ok(existing_records)
}
