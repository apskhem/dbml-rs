use std::fs;
use std::io::Result;
use std::path::{
  Path,
  PathBuf,
};

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
  if fs::metadata(OUT_DIR).is_err() {
    fs::create_dir(OUT_DIR)?;
  }

  Ok(())
}

/// Run with UPDATE_DBML_OUTPUT=1 to update the expected output files
/// e.g. (on Linux or macOS)
/// UPDATE_DBML_OUTPUT=1 cargo test
#[test]
fn parse_dbml_unchecked() -> Result<()> {
  create_out_dir()?;

  let testing_dbml_paths = read_dbml_dir("tests/dbml")?;

  for path in testing_dbml_paths {
    let content = fs::read_to_string(&path)?;
    let parsed =
      dbml_rs::parse_dbml_unchecked(&content).unwrap_or_else(|err| panic!("{}", err.with_path(path.to_str().unwrap())));

    let out_content = format!("{:#?}", parsed);

    let mut out_file_path = path.clone();
    out_file_path.set_extension("ron");
    let out_file_name = out_file_path.file_name().unwrap().to_str().unwrap();
    let out_file_path = format!("{}/{}", OUT_DIR, out_file_name);

    let update = match std::env::var("UPDATE_DBML_OUTPUT") {
      Ok(v) => v == "1",
      _ => false
    };
    if update {
      fs::write(out_file_path, out_content)?;
    } else {
      let expected = fs::read_to_string(&out_file_path).or_else(|e| {
        match e.kind() {
          std::io::ErrorKind::NotFound => {
            Ok("no output file".to_string())
          },
          e => Err(e)
        }
      })?;
      assert_eq!(out_content, expected, "Unexpected output for {:?}", path);
    }
  }

  Ok(())
}
  
#[test]
fn parse_dbml_validator() -> Result<()> {
  let testing_dbml_paths = read_dbml_dir("tests/dbml/validator")?;

  for path in testing_dbml_paths {
    let content = fs::read_to_string(&path)?;

    if let Ok(_) = dbml_rs::parse_dbml(&content) {
      panic!("{:?}: validation unexpected", path)
    }
  }

  Ok(())
}

#[test]
fn parse_dbml_checked_tmp() -> Result<()> {
  let path = "tests/dbml/checked/sample_tmp.dbml";
  let content = fs::read_to_string(path)?;

  dbml_rs::parse_dbml(&content).unwrap_or_else(|err| panic!("{}", err.with_path(path)));

  Ok(())
}
