use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;

pub enum Tree {
  BlogEntry { 
    name: String,
    hash: String
  },
  TreeEntry {
    name: String,
    hash: String,
    children: Vec<Tree>,
  }
}

pub struct Blob {
  pub hash: String,     // hash value of file content
  pub data: Vec<u8> ,   // file content(blob)
}

impl Blob {
  // @path: full path of the file that will be indexed
  pub fn from_path(path: &PathBuf) -> io::Result<Blob> {
    let mut file = File::open(path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    
    // do hash file content using SHA-1
    let mut sha = Sha1::new();
    sha.input(&bytes);

    Ok(
      Blob {
        hash: sha.result_str(),
        data: bytes,
      }
    )
  }
}
