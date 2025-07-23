use std::{
    fs::File,
    io::{BufReader, Write}, path::Path,
};

use crate::model::List;

pub fn store(tasks: &List, path: &str) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(tasks).expect("Failed to serialize List of items");
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn read(path: &str) -> std::io::Result<List> {
    if !Path::new(path).exists() {
        let _file = File::create(path)?;
        return Ok(List::new());
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let list = serde_json::from_reader(reader)?;
    Ok(list)
}
