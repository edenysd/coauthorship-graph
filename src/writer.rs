use std::{collections::HashMap, fs::File};

use serde_json::Error;

pub fn write<K: ToString, V: ToString>(writter: File, map: &HashMap<K, V>) -> Result<(), Error> {
    let mut n_map = HashMap::<String, String>::new();

    for (k, v) in map {
        n_map.insert(k.to_string(), v.to_string());
    }

    serde_json::to_writer_pretty(&writter, &n_map)?;
    Ok(())
}
