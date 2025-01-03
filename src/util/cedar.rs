use cedar_policy::*;
use std::{
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

#[inline]
pub fn read_policy_set_from_file<P: AsRef<Path>>(path: P) -> PolicySet {
    let content = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {:?} error: {}", path.as_ref(), e));
    PolicySet::from_str(&content)
        .unwrap_or_else(|e| panic!("parse {:?} error: {}", path.as_ref(), e))
}

#[inline]
pub fn read_entities_from_file<P: AsRef<Path>>(path: P) -> Entities {
    let f = fs::File::open(&path).unwrap();
    Entities::from_json_file(f, None)
        .unwrap_or_else(|e| panic!("parse {:?} error: {}", path.as_ref(), e))
}

#[inline]
pub fn read_schemas_from_file<P: AsRef<Path>>(path: P) -> Schema {
    let f = fs::File::open(&path).unwrap();
    let (schema, _) = Schema::from_cedarschema_file(f)
        .unwrap_or_else(|e| panic!("parse {:?} error: {}", path.as_ref(), e));
    schema
}

#[inline]
pub fn policy_path(file: &str) -> PathBuf {
    fs::canonicalize("resource/policies/".to_string() + file).unwrap()
}

#[inline]
pub fn entity_path(file: &str) -> PathBuf {
    fs::canonicalize("resource/entities/".to_string() + file).unwrap()
}

#[inline]
pub fn schema_path(file: &str) -> PathBuf {
    fs::canonicalize("resource/schemas/".to_string() + file).unwrap()
}

#[inline]
pub fn atom_to_action(atom: &str) -> EntityUid {
    EntityUid::from_str(format!(r#"Action::"{atom}""#).as_str()).unwrap()
}
