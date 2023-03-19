use crypto::digest::Digest;
use crypto::sha1::Sha1;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::BTreeMap;
use std::io::Write;

use crate::error::TgitError;
use crate::file::FileService;
use crate::index::Index;

pub struct Commit {
  pub hash: Option<String>,                 // hash value of file content
  pub data: Option<Vec<u8>>,                // file content(blob)
  pub parent: Option<String>,               // is the parent hash
  pub files: BTreeMap<String, String>       // map of commited files
}

impl Commit {
  // @parent: ex => Some(Commit { hash: Some("1edf3cb6c56fb58fb214d9b150e37bf68725f167"), data: None, parent: Some("9397a762d614f32a6720887d678ea2f9f478bc30"), files: {} })
  pub fn new(parent: Option<&Commit>) -> Commit {
    let mut commit = Commit {
      hash: None,
      data: None,
      parent: match parent {
        Some(&Commit { 
          hash: Some(ref hash),
          ..
        }) => Some(hash.to_string()),
        _ => None
      },
      files: BTreeMap::new(),
    };

    for (ref hash, ref path) in parent.iter().flat_map(|p| p.files.iter()) {
      commit.files.insert(hash.to_string(), path.to_string());
    }

    commit
  }

  pub fn add_from_index(&mut self, index: &Index) {
    for (ref hash, ref path) in index.hashtree.iter() {
      self.files.insert(hash.to_string(), path.to_string());
    }
  }

  // @hash: parent commit of current branch, @input: the content of parent commit hash(ex: parent faac27718188671d85f5dbd040ea4b5077cb4635)
  // the parent in the content of parent is the grandparent
  pub fn from_string(hash: &str, input: &str) -> Result<Commit, TgitError> {
    let mut commit = Commit::new(None);
    commit.hash = Some(hash.to_string());
    lazy_static! {
      static ref PARENT: Regex = Regex::new(r"parent ([0-9a-f]{40})").unwrap();
      static ref BLOB: Regex = Regex::new(r"blob ([0-9a-f]{40}) (.*)").unwrap();
    }

    for line in input.lines() {
      // if there is parent commit
      if let Some(ref caps) = PARENT.captures(line) {
        commit.parent = Some(caps.get(1).unwrap().as_str().to_string());
      }
      // if there is blob hash
      if let Some(ref caps) = BLOB.captures(line) {
        let hash = caps.get(1).unwrap().as_str();
        let path = caps.get(2).unwrap().as_str();
        commit.files.insert(hash.to_string(), path.to_string());
      }
    }

    Ok(commit)
  }

  pub fn print(&self) {
    if let Some(ref p) = self.parent {
      println!("parent {}", p);
    }
    for (ref hash, ref path) in self.files.iter() {
      println!("blob {} {}", hash, path);
    }
  }

  pub fn update(&mut self) {
    let mut data = Vec::new();

    if let Some(ref p) = self.parent {
      writeln!(&mut data, "parent {}", p).unwrap();
    }

    for (ref hash, ref path) in self.files.iter() {
      writeln!(&mut data, "blob {} {}", hash, path).unwrap();
    }
    
    let mut sha = Sha1::new();
    sha.input(&data);
    self.hash = Some(sha.result_str());
    self.data = Some(data);
  }
}


pub fn commit() -> Result<(), TgitError>{
  let fs = FileService::new()?;
  let head_ref = fs.get_head_ref()?;
  let parent_hash = FileService::get_hash_from_ref(&head_ref);
  let mut index = Index::new(&fs.root_dir)?;

  // there is parent commit, or not
  let parent = match parent_hash {
    Some(ref h) => Some(fs.read_commit(h)?),
    None => None,
  };

  let mut commit = Commit::new(parent.as_ref());
  parent.map(|p| p.print());
  commit.add_from_index(&index);
  commit.print();

  fs.write_commit(&mut commit)?;
  index.clear()?;
  Ok(())
}
