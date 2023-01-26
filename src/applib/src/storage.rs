use memmap::Mmap;
use std::{fs::File, io, path::Path};

#[derive(thiserror::Error, Debug)]
pub enum UniversalContainerError {
    #[error("unable to open save file: {0}")]
    InvalidFile(io::Error),

    #[error("unable to memory map file: {0}")]
    Mmap(io::Error),

    #[error("unable to parse file: {0}")]
    Parse(#[from] asar_save::AsarError),
}

pub fn to_universal_container<P: AsRef<Path>>(
    fp: P,
    filename: &str,
) -> Result<Vec<u8>, UniversalContainerError> {
    let f = File::open(fp.as_ref()).map_err(UniversalContainerError::InvalidFile)?;
    let mmap = unsafe { Mmap::map(&f).map_err(UniversalContainerError::Mmap)? };
    let out = asar_save::create_asar(
        &mmap[..],
        asar_save::Game::Eu4,
        &asar_save::Metadata { filename },
    )?;
    Ok(out)
}
