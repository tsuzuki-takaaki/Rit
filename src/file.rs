use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{ Read, Write };
use std::path::{ Path, PathBuf };

use crate::commit::Commit;
use crate::error::TgitError;
use crate::types::Blob;

pub struct FileService {
  pub root_dir: PathBuf,      // is the project dir
  pub tgit_dir: PathBuf,      // is .tgit dir
  pub object_dir: PathBuf,    // is .tgit/objects dir
}

impl FileService {
  pub fn new() -> Result<FileService, TgitError> {
    // after root dir is determined, tgit_dir and object_dir will be determined(only depend on root_dir)
    let root_dir = FileService::find_root()?;
    let tgit_dir = root_dir.join(".tgit").to_path_buf();
    let object_dir = tgit_dir.join("objects").to_path_buf();
    Ok(FileService { root_dir, tgit_dir, object_dir })
  }

  // search root directory(search direcoty that has .tgit directory <=> .tgit parents)
  fn find_root() -> Result<PathBuf, TgitError> {
    let mut current_dir = env::current_dir()?;
    loop {
      if FileService::is_tgit(&current_dir) {
        return Ok(current_dir);
      }
      if !current_dir.pop() {
        return Err(TgitError::NoDirectory);
      }
    }
  }

  // check path is the target(root) directory
  fn is_tgit<P>(path: P) -> bool
  where
    P: Sized + AsRef<Path>
  {
    // as_ref() will return &Path (ref: https://qiita.com/DeliciousBar/items/de686ade39b00960df61)
    path.as_ref().join(".tgit").exists()
  }

  // get .tgit/HEAD content(it is the place of head(current branch))
  // and return the path: .tgit/refs/heads/{master, development, feature}
  // each files have branch's commit hash
  pub fn get_head_ref(&self) -> io::Result<PathBuf> {
    let mut head_file = File::open(self.root_dir.join(".tgit/HEAD"))?;
    let mut ref_path = String::new();
    head_file.read_to_string(&mut ref_path)?;
    let ref_path = ref_path.split_off(6);
    Ok(self.tgit_dir.join(ref_path))
  }

  // get parent hash if there are
  // if there are not, return None
  pub fn get_hash_from_ref(ref_path: &PathBuf) -> Option<String> {
    match File::open(ref_path) {
      Ok(ref mut f) => {
        let mut hash = String::new();
        f.read_to_string(&mut hash).unwrap();
        Some(hash)
      },
      Err(_) => None,
    }
  }
  
  pub fn write_blob(&self, blob: &Blob) -> io::Result<()> {
    self.write_object(&blob.hash, &blob.data)
  }

  // @hash: parent commit of current branch
  pub fn read_commit(&self, hash: &str) -> Result<Commit, TgitError> {
    Commit::from_string(hash, &self.read_object(hash)?)
  }
  
  pub fn write_commit(&self, commit: &mut Commit) -> io::Result<()> {
    commit.update();

    match commit {
      &mut Commit {
        hash: Some(ref hash),
        data: Some(ref data),
        ..
      } => {
        self.write_object(hash, data)?;
        let head = self.get_head_ref()?;
        let mut head_file = File::create(&head)?;
        head_file.write_all(hash.as_bytes())?;
      },
      _ => panic!("Commit should have data and hash"),
    }
    Ok(())
  }

  pub fn write_object(&self, hash: &str, data: &Vec<u8>) -> io::Result<()> {
    // determine directory name(&hash[..2] is just for directory name)
    let blob_dir = self.object_dir.join(&hash[..2]);
    if !blob_dir.exists() {
      fs::create_dir(&blob_dir)?;
    }
    // determin file name(&hash[2..] is just for file name)
    let blob_filename = blob_dir.join(&hash[2..]);
    let mut blob_f = File::create(&blob_filename)?;
    blob_f.write_all(data)?;

    Ok(())
  }

  // @hash: parent commit of current branch
  pub fn read_object(&self, hash: &str) -> io::Result<String> {
    let mut data = String::new();
    // get the path of target parent hash
    let object_filename = self.object_dir.join(&hash[..2]).join(&hash[2..]);
    let mut object_file = File::open(&object_filename)?;
    object_file.read_to_string(&mut data)?;
    Ok(data)
  }
}
