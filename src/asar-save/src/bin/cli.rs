use std::{
    error::Error,
    io::{Read, Write},
};

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let mut lock = stdin.lock();
    let mut buf = Vec::new();
    lock.read_to_end(&mut buf)?;

    let out = asar_save::create_asar(
        &buf,
        asar_save::Game::Vic3,
        &asar_save::Metadata {
            filename: "save-game",
        },
    )?;

    let stdout = std::io::stdout();
    let mut stdout_lock = stdout.lock();
    stdout_lock.write_all(&out)?;

    Ok(())
}
