use std::{env, thread::current};
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use sha1::{Digest, Sha1};

fn main() -> std::io::Result<()>{
    let filename = "first.txt";

    let mut path = env::current_dir()?;
    path.push(PathBuf::from(filename));

    let mut f = File::open(path)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;

    let blob_content = format!("blob {}\0{}", content.len(), content);

    println!("blob content: {}", blob_content);

    let blob_hash = Sha1::digest(blob_content.as_bytes());
    println!("blob hash: {:x}", blob_hash);

    Ok(())
}
