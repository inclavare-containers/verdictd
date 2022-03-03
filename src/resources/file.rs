use std::fs;
use std::io::prelude::*;
use std::path::Path;

pub fn export_string(name: &str) -> Result<String, String> {
    fs::File::open(name)
        .map_err(|e| e.to_string())
        .and_then(|mut file| {
            let mut contents = String::new();
            let res = file.read_to_string(&mut contents)
                .map_err(|e| e.to_string())
                .and_then(|_| Ok(contents));
            res
        })
}

pub fn export_raw(name: &str) -> Result<Vec<u8>, String> {
    fs::File::open(name)
        .map_err(|e| e.to_string())
        .and_then(|mut file| {
            let mut contents = Vec::new();
            let res = file.read_to_end(&mut contents)
                .map_err(|e| e.to_string())
                .and_then(|_| Ok(contents));
            res
        })
}

pub fn export_base64(name: &str) -> Result<String, String> {
    export_raw(name)
        .map_err(|e| e.to_string())
        .and_then(|contents| {
            Ok(base64::encode(contents))
        })
}

pub fn write(src: &str, content: &str) -> Result<(), String> {
    // Open the file in write-only mode
    // If the file with the same name already exists, it will be overwritten
    fs::File::create(src)
        .map_err(|e| e.to_string())
        .and_then(|mut file| {
            let res = file.write_all(content.as_bytes())
                .map_err(|e| e.to_string())
                .and_then(|_| Ok(()));
            res
        })
}

pub fn set(name: &str, content: &str) -> Result<(), String> {
    let src = name;
    let bak = name.clone().to_owned() + ".bak";

    if Path::new(&src).exists() {
        fs::copy(&src, &bak).unwrap();
    }

    write(&src, content)
        .map_err(|e| {
            if Path::new(&bak).exists() {
                fs::copy(&bak, &src).unwrap();
            }
            e
        })
        .and_then(|_| {
            if Path::new(&bak).exists() {
                fs::remove_file(&bak).unwrap();
            }
            Ok(())
        })
}

pub fn size(name: &str) -> Result<usize, String> {
    fs::metadata(name)
        .map_err(|e| e.to_string())
        .and_then(|metadata| Ok(metadata.len() as usize))
}
