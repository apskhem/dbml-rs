use std::{
  fs,
  io::Result,
  path::{Path, PathBuf},
};

const DBML_DIR: &str = "tests/dbml";
const OUT_DIR: &str = "tests/out";

fn read_dbml_dir<P: AsRef<Path>>(dir_path: P) -> Result<Vec<PathBuf>> {
  let mut out = vec![];
  let entries = fs::read_dir(dir_path)?;

  for entry in entries {
    let file_path = entry?.path();

    if file_path.is_file() {
      out.push(file_path);
    }
  }

  Ok(out)
}

fn create_out_dir() -> Result<()> {
  if !fs::metadata(OUT_DIR).is_ok() {
    fs::create_dir(OUT_DIR)?;
  }

  Ok(())
}

#[test]
fn parse_dbml() -> Result<()> {
  create_out_dir()?;

  let testing_dbml_files = read_dbml_dir(DBML_DIR)?;

  for path in testing_dbml_files {
    let content = fs::read_to_string(&path)?;
    let parsed =
      dbml_rs::parser::parse(&content).unwrap_or_else(|err| panic!("{:?} {}", path, err));

    let out_content = format!("{:#?}", parsed);

    let mut out_file_path = path.clone();
    out_file_path.set_extension("ron");
    let out_file_name = out_file_path.file_name().unwrap().to_str().unwrap();
    let out_file_path = format!("{}/{}", OUT_DIR, out_file_name);

    fs::write(out_file_path, out_content)?;
  }

  Ok(())
}
