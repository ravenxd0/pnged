use std::fs::OpenOptions;
use std::io::Write;
use std::str::FromStr;

use crate::args::{Cli, Commands};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::Result;
use clap::Parser;

pub fn run() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::Encode {
            path,
            chunk_type,
            message,
            mut output_file,
        } => {
            let mut png = Png::from_file(&path)?;
            let chunk_type = check_chunk_type_length(&chunk_type)?;
            let chunk = Chunk::new(chunk_type, message.into_bytes());
            png.add_message(chunk);

            if output_file.is_none() {
                output_file = if let Some(file) = path.to_str() {
                    Some(file.to_string())
                } else {
                    Some("output.png".to_string())
                };
            }

            let mut file = OpenOptions::new()
                .create(true)
                .truncate(true)
                .read(true)
                .write(true)
                .open(output_file.unwrap())?;

            file.write_all(&png.as_bytes())?;
        }
        Commands::Decode { path, chunk_type } => {
            check_chunk_type_length(&chunk_type)?;

            let png = Png::from_file(path)?;
            let chunk = png.chunk_by_type(&chunk_type);

            match chunk {
                Some(chunk) => {
                    println!("Message: {}", chunk.data_as_string()?);
                }
                None => {
                    println!("No Chunk with Chunk Type `{}`", chunk_type);
                }
            }
        }
        Commands::Remove { path, chunk_type } => {
            check_chunk_type_length(&chunk_type)?;

            let mut png = Png::from_file(&path)?;
            let removed_chunk = png.remove_chunk(&chunk_type)?;

            println!(
                "Chunk is Remove From File {}\n{removed_chunk} : ",
                path.to_string_lossy()
            );

            let mut file = OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(path)?;
            file.write_all(&png.as_bytes())?;
        }
        Commands::Print { path } => {
            let png = Png::from_file(&path)?;
            println!("All Non-Critical, Private and Safe to Copy Chunk Types Are:");
            for chunk in png.chunks().iter() {
                let chunk_type = chunk.chunk_type();

                if !chunk_type.is_critical()
                    && !chunk_type.is_public()
                    && chunk_type.is_safe_to_copy()
                {
                    println!("{}", chunk);
                }
            }
        }
    }

    Ok(())
}

fn check_chunk_type_length(chunk_type: &str) -> Result<ChunkType> {
    if chunk_type.len() != 4 {
        let message = "INVALID CHUNK TYPE (MUST BE OF LENGTH 4)";
        return Err(message.into());
    }

    let chunk_type = ChunkType::from_str(chunk_type)?;

    if !chunk_type.is_critical() && !chunk_type.is_public() && chunk_type.is_safe_to_copy() {
        Ok(chunk_type)
    } else {
        Err("Chunk Type is Not Ancillary or Private or Safe to Copy".into())
    }
}
