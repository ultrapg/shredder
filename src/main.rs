use rand::{Rng, RngCore};
use std::env;
use std::fs;
use std::io::{self, Seek, SeekFrom, Write};
use std::path::Path;

const BUFFER_SIZE: usize = 64 * 1024; // 64 KB
const DEFAULT_PASSES: u32 = 3;

fn shred(path: &Path, passes: u32) -> io::Result<()> {
    let metadata = fs::metadata(path)
        .map_err(|e| io::Error::new(e.kind(), format!("{}: {}", path.display(), e)))?;

    if !metadata.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} is not a regular file", path.display()),
        ));
    }

    let file_len = metadata.len();
    let mut file = fs::OpenOptions::new().write(true).open(path)?;

    if passes == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "passes must be greater than 0",
        ));
    }

    let mut rng = rand::thread_rng();
    let mut buffer = vec![0u8; BUFFER_SIZE];

    for pass in 1..=passes {
        file.seek(SeekFrom::Start(0))?;
        let mut remaining = file_len;
        while remaining > 0 {
            let chunk_size = remaining.min(BUFFER_SIZE as u64) as usize;
            rng.fill_bytes(&mut buffer[..chunk_size]);
            file.write_all(&buffer[..chunk_size])?;
            remaining -= chunk_size as u64;
        }
        file.sync_all()?;
        println!("Pass {}/{} completed", pass, passes);
    }

    drop(file);

    // Rename to a random string to erase filesystem metadata
    let parent = path.parent().unwrap_or(Path::new("."));
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz0123456789".chars().collect();
    let random_path = loop {
        let random_name: String = (0..12).map(|_| {
            let idx = rng.gen_range(0..chars.len());
            chars[idx]
        }).collect();
        let candidate = parent.join(&random_name);
        match candidate.try_exists() {
            Ok(false) => break candidate,
            Ok(true) => continue,
            Err(_) => break candidate,
        }
    };
    fs::rename(path, &random_path)?;

    fs::remove_file(&random_path)?;
    println!("File successfully shredded");

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 3 {
        eprintln!("Usage: shredder <file> [passes]");
        std::process::exit(1);
    }

    let path = Path::new(&args[1]);
    let passes: u32 = args
        .get(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_PASSES);

    match shred(path, passes) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
