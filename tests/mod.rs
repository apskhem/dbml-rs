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

fn create_out_dir(sub_dir: Option<&PathBuf>) -> Result<()> {
  if fs::metadata(OUT_DIR).is_err() {
    fs::create_dir(OUT_DIR)?;
  }
  if let Some(sub_dir) = sub_dir {
    let path = Path::new(OUT_DIR).join(sub_dir);
    if fs::metadata(&path).is_err() {
      fs::create_dir(path)?;
    }
  }

  Ok(())
}

/// Run with UPDATE_DBML_OUTPUT=1 to update the expected output files
/// e.g. (on Linux or macOS)
/// UPDATE_DBML_OUTPUT=1 cargo test
fn update_expected() -> bool {
  match std::env::var("UPDATE_DBML_OUTPUT") {
    Ok(v) => v == "1",
    _ => false,
  }
}

fn compare_parsed_with_expected(sub_dir: Option<impl Into<PathBuf>>, update: bool) -> Result<()> {
  let sub_dir = sub_dir.map(Into::into);

  create_out_dir(sub_dir.as_ref())?;
  let mut source_dir = Path::new("tests/dbml").to_path_buf();
  let mut out_file_dir = Path::new(OUT_DIR).to_path_buf();
  if let Some(sub_dir) = sub_dir {
    source_dir.push(&sub_dir);
    out_file_dir.push(&sub_dir);
  }

  let testing_dbml_paths = read_dbml_dir(source_dir)?;

  for path in testing_dbml_paths {
    let content = fs::read_to_string(&path)?;
    let parsed =
      dbml_rs::parse_dbml_unchecked(&content).unwrap_or_else(|err| panic!("{}", err.with_path(path.to_str().unwrap())));

    let out_content = format!("{:#?}", parsed);

    let mut out_file_path = path.clone();
    out_file_path.set_extension("ron");
    let out_file_name = out_file_path.file_name().unwrap().to_str().unwrap();
    let out_file_path = out_file_dir.join(out_file_name);

    if update {
      fs::write(out_file_path, out_content)?;
    } else {
      let expected = fs::read_to_string(&out_file_path).or_else(|e| {
        match e.kind() {
          std::io::ErrorKind::NotFound => Ok("no output file".to_string()),
          e => Err(e),
        }
      })?;
      assert_eq!(out_content, expected, "Unexpected output for {:?}", path);
    }
  }

  Ok(())
}

#[test]
fn parse_dbml_root() -> Result<()> {
  compare_parsed_with_expected(None::<PathBuf>, update_expected())
}

#[test]
fn parse_dbml_mysql_importer() -> Result<()> {
  compare_parsed_with_expected(Some("mysql_importer"), update_expected())
}

#[test]
fn parse_dbml_mssql_importer() -> Result<()> {
  compare_parsed_with_expected(Some("mssql_importer"), update_expected())
}

#[test]
fn parse_dbml_pgsql_importer() -> Result<()> {
  compare_parsed_with_expected(Some("postgres_importer"), update_expected())
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
