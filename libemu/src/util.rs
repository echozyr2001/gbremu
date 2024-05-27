use std::{
  fs::File,
  io::{Read, Write},
  path::Path,
};

use crate::error::Error;

pub fn read_file(path: &str) -> Result<Vec<u8>, Error> {
  let mut file =
    File::open(path).map_err(|_| Error::CustomError(format!("Failed to open file: {}", path)))?;
  let mut data = Vec::new();
  file
    .read_to_end(&mut data)
    .map_err(|_| Error::CustomError(format!("Failed to read from file: {}", path)))?;
  Ok(data)
}

pub fn write_file(path: &str, data: &[u8]) -> Result<(), Error> {
  let mut file = File::create(path)
    .map_err(|_| Error::CustomError(format!("Failed to create file: {}", path)))?;
  file
    .write_all(data)
    .map_err(|_| Error::CustomError(format!("Failed to write to file: {}", path)))?;
  file
    .flush()
    .map_err(|_| Error::CustomError(format!("Failed to flush file: {}", path)))?;
  Ok(())
}

pub fn replace_ext(path: &str, new_extension: &str) -> Option<String> {
  let file_path = Path::new(path);
  let parent_dir = file_path.parent()?;
  let file_stem = file_path.file_stem()?;
  let file_extension = file_path.extension()?;
  if file_stem == file_extension {
    return None;
  }
  let new_file_name = format!("{}.{}", file_stem.to_str()?, new_extension);
  let new_file_path = parent_dir.join(new_file_name);
  Some(String::from(new_file_path.to_str()?))
}
