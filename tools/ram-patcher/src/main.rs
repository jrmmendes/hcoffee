//! Linux CLI port of SilentPatch's `largeaddress.exe`.
//!
//! Toggles the `IMAGE_FILE_LARGE_ADDRESS_AWARE` (0x0020) bit in the COFF
//! `Characteristics` field of a 32-bit PE image, so the game may address
//! more than 2 GB of RAM (3 GB on 32-bit Windows with the `/3GB` switch,
//! 4 GB on 64-bit Windows).

use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::Parser;

/// `IMAGE_FILE_LARGE_ADDRESS_AWARE`
const LARGE_ADDRESS_AWARE: u16 = 0x0020;

const E_LFANEW_OFFSET: usize = 0x3c;
const COFF_HEADER_SIZE: usize = 20;
const COFF_CHARACTERISTICS_OFFSET: usize = 18;

#[derive(Parser, Debug)]
#[command(
    name = "largeaddress",
    about = "Toggles /LARGEADDRESSAWARE on GTA III/VC/SA executables",
    long_about = "Toggles the /LARGEADDRESSAWARE flag on GTA III/VC/SA executables.\n\
                  It will allow the games to use 3GB of RAM on a 32-bit operating\n\
                  system with a /3GB switch on, and 4GB on 64-bit machines.\n\
                  Linux CLI port of SilentPatch's tool created by Silent."
)]
struct Cli {
    /// Path to the GTA executable (gta3.exe / gta-vc.exe / gta_sa.exe).
    exe: PathBuf,

    /// Answer "yes" to the confirmation prompt (as if pressing Y).
    #[arg(short = 'y', long = "yes")]
    assume_yes: bool,

    /// Do not create a `.bak` backup of the original executable.
    #[arg(short = 'n', long = "no-backup")]
    no_backup: bool,
}

#[derive(Debug)]
enum PeError {
    NotEnoughData,
    NotMz,
    NotPe,
    OutOfBounds,
}

impl std::fmt::Display for PeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            PeError::NotEnoughData => "file is too small to be a valid PE image",
            PeError::NotMz => "invalid file: missing 'MZ' DOS header",
            PeError::NotPe => "invalid file: missing 'PE\\0\\0' signature",
            PeError::OutOfBounds => "invalid file: PE header points outside the file",
        };
        f.write_str(msg)
    }
}

impl std::error::Error for PeError {}

/// Byte offset of the COFF `Characteristics` field within `data`.
fn characteristics_offset(data: &[u8]) -> Result<usize, PeError> {
    if data.len() < 0x40 {
        return Err(PeError::NotEnoughData);
    }
    if &data[0..2] != b"MZ" {
        return Err(PeError::NotMz);
    }

    let e_lfanew = u32::from_le_bytes(
        data[E_LFANEW_OFFSET..E_LFANEW_OFFSET + 4]
            .try_into()
            .unwrap(),
    ) as usize;

    // PE signature + COFF header must fit, and the field must be in bounds.
    let pe_start = e_lfanew;
    let characteristics = pe_start + 4 + COFF_CHARACTERISTICS_OFFSET;
    if characteristics + 2 > data.len() || pe_start + 4 + COFF_HEADER_SIZE > data.len() {
        return Err(PeError::OutOfBounds);
    }

    if &data[pe_start..pe_start + 4] != b"PE\0\0" {
        return Err(PeError::NotPe);
    }

    Ok(characteristics)
}

fn read_flag(data: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes(data[offset..offset + 2].try_into().unwrap())
}

fn ask_confirmation(prompt: &str) -> io::Result<bool> {
    print!("{prompt}");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let answer = input.trim();
    Ok(answer.is_empty() || answer.eq_ignore_ascii_case("y") || answer.eq_ignore_ascii_case("yes"))
}

fn backup_path(path: &Path) -> PathBuf {
    let mut name = path.as_os_str().to_os_string();
    name.push(".bak");
    PathBuf::from(name)
}

fn run(cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    let path = &cli.exe;

    println!("Checking the input file...");

    let mut data = fs::read(path)
        .map_err(|e| format!("could not read '{}': {e}", path.display()))?;

    let offset = characteristics_offset(&data)
        .map_err(|e| format!("'{}' is not a valid executable: {e}", path.display()))?;

    let characteristics = read_flag(&data, offset);
    if characteristics & LARGE_ADDRESS_AWARE != 0 {
        return Err("ERROR: This file has a /LARGEADDRESSAWARE flag already!".into());
    }

    if !cli.assume_yes
        && !ask_confirmation("The file is valid and can be patched. Proceed? [Y/n] ")?
    {
        println!("Aborted.");
        return Ok(());
    }

    if !cli.no_backup {
        println!("Backing up an old EXE file...");
        let backup = backup_path(path);
        if let Err(e) = fs::copy(path, &backup) {
            eprintln!("WARNING: Could not create a backup of your EXE file ({e}).");
            if !cli.assume_yes
                && !ask_confirmation("WARNING: Could not create a backup of your EXE file, proceed anyway? [Y/n] ")?
            {
                println!("Aborted.");
                return Ok(());
            }
        }
    }

    println!("Patching...");
    let patched = characteristics | LARGE_ADDRESS_AWARE;
    data[offset..offset + 2].copy_from_slice(&patched.to_le_bytes());
    fs::write(path, &data)
        .map_err(|e| format!("could not write '{}': {e}", path.display()))?;

    println!("File successfully patched!");
    Ok(())
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(&cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fake_pe(characteristics: u16) -> Vec<u8> {
        let mut data = vec![0u8; 0x100];
        data[0..2].copy_from_slice(b"MZ");
        // e_lfanew -> 0x80
        data[E_LFANEW_OFFSET..E_LFANEW_OFFSET + 4].copy_from_slice(&0x80u32.to_le_bytes());
        data[0x80..0x84].copy_from_slice(b"PE\0\0");
        let off = 0x80 + 4 + COFF_CHARACTERISTICS_OFFSET;
        data[off..off + 2].copy_from_slice(&characteristics.to_le_bytes());
        data
    }

    #[test]
    fn locates_characteristics() {
        let data = fake_pe(0x0102);
        let off = characteristics_offset(&data).unwrap();
        assert_eq!(read_flag(&data, off), 0x0102);
    }

    #[test]
    fn rejects_non_mz() {
        let mut data = fake_pe(0);
        data[0] = b'Z';
        assert!(matches!(characteristics_offset(&data), Err(PeError::NotMz)));
    }

    #[test]
    fn rejects_bad_signature() {
        let mut data = fake_pe(0);
        data[0x80] = b'X';
        assert!(matches!(characteristics_offset(&data), Err(PeError::NotPe)));
    }

    #[test]
    fn detects_existing_flag() {
        let data = fake_pe(LARGE_ADDRESS_AWARE);
        let off = characteristics_offset(&data).unwrap();
        assert_ne!(read_flag(&data, off) & LARGE_ADDRESS_AWARE, 0);
    }

    #[test]
    fn backup_path_appends_bak() {
        assert_eq!(backup_path(Path::new("/g/gta_sa.exe")), Path::new("/g/gta_sa.exe.bak"));
    }
}
