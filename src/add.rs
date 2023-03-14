// for add command
use std::env;

use crate::error::TgitError;
use crate::file::FileService;
use crate::index::Index;
use crate::types::Blob;

// @add_data: the files path will be indexed(already and new one)
pub fn add_all(add_data: &Vec<&str>) -> Result<(), TgitError> {
  let file_service = FileService::new()?;
  let current_dir = env::current_dir()?;
  let mut index = Index::new(&file_service.root_dir)?;
  
  for file in add_data {
    let full_path = current_dir.join(file);
    // make blob and do hash
    let blob = Blob::from_path(&full_path)?;
    // update files in .tgit/objects
    file_service.write_blob(&blob)?;

    let relative_path = full_path.strip_prefix(&file_service.root_dir).unwrap();
    index.update(&relative_path.to_str().unwrap(), &blob.hash);
  }
  // update .tgit/index
  index.write()?;
  Ok(())
}
