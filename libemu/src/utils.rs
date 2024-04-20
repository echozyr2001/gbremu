use std::{fs::File, io::Read};

use crate::error::Error;

pub fn read_data(path: &str) -> Result<Vec<u8>, Error> {
  let mut file =
    File::open(path).map_err(|_| Error::CustomError(format!("Filed to open file: {}", path)))?;

  let mut data = Vec::new();
  file.read_to_end(&mut data).unwrap();
  Ok(data)
}
