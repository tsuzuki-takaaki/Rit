// indexing each file data in .tgit/index
// (key, value) = (path, hashtree)
// sample:
// src/main.rs d55509893a51544c39790f1f6f6f2994681ae2ae
use std::collections::BTreeMap;
use std::fs::File;
use std::io;
use std::io::{ BufRead, BufReader, Write };
use std::path::PathBuf;

use crate::error::TgitError;

pub struct Index {
  pub path: PathBuf,                          // project_dir/.tgit/index
  pub hashtree: BTreeMap<String, String>,     // controll .tgit/index
}

impl Index {
  // @root_dir: the project dir
  pub fn new(root_dir: &PathBuf) -> Result<Index, TgitError> {
    let mut index = Index {
      path: root_dir.join(".tgit").join("index"),
      hashtree: BTreeMap::new(),
    };

    if !index.path.exists() {
      return Ok(index);
    }

    // read whole .tgit/index file
    let file = BufReader::new(File::open(&index.path)?);

    // read .tgit/index file line by line(already indexed files)
    for line in file.lines() {
      let ln = line?;
      // blob => vec![key, value]
      let blob: Vec<&str> = ln.split(' ').collect();
      if blob.len() != 2 {
        return Err(TgitError::InvalidIndex);
      }
      index.update(blob[0], blob[1]);
    }
    Ok(index)
  }

  pub fn update(&mut self, path: &str, hash: &str) {
    // update Index.hashtree(not update .tgit/index)
    self.hashtree.insert(path.to_string(), hash.to_string());
  }

  pub fn print(&self) {
    for (ref hash, ref path) in self.hashtree.iter() {
      println!("{} {}", hash, path);
    }
  }

  pub fn write(&self) -> io::Result<()> {
    let mut index = File::create(&self.path)?;
    for (ref path, ref hash) in self.hashtree.iter() {
      println!("path: {}, hash: {}, index: {:?}", path, hash, index);
      writeln!(&mut index, "{} {}", path, hash);
    }
    Ok(())
  }

  pub fn clear(&mut self) -> io::Result<()> {
    self.hashtree = BTreeMap::new(); // recreate the tree
    self.write()
  }
}
