use std::io::{Read, Seek};
use std::{
    fs::{self, File},
    io::Write,
};

pub fn load_file(path: &str) -> std::io::Result<File> {
    Ok(fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(path)?)
}
pub fn read_file_to_string(file: &mut File) -> std::io::Result<String> {
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    Ok(buf)
}

pub fn write_all_to_file(file: &mut File, content: &str) -> std::io::Result<()> {
    file.seek(std::io::SeekFrom::Start(0))?;
    file.set_len(0)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
