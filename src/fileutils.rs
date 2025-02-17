use std::{path::{Path, PathBuf}, fs::{File, OpenOptions}, io::Seek};

use chrono::{DateTime, Local, Datelike, Timelike};
use dse::dtype::DSEError;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn get_file_last_modified_date_with_default<P: AsRef<Path>>(input_file_path: P) -> Result<(u16, u8, u8, u8, u8, u8, u8), DSEError> {
    if let Ok(time) = std::fs::metadata(&input_file_path)?.modified() {
        let dt: DateTime<Local> = time.into();
        Ok((
            dt.year() as u16,
            dt.month() as u8,
            dt.day() as u8,
            dt.hour() as u8,
            dt.minute() as u8,
            dt.second() as u8,
            (dt.nanosecond() / 10_u32.pow(7)) as u8
        ))
    } else {
        Ok((
            2008,
            11,
            16,
            13,
            40,
            57,
            3
        ))
    }
}

pub fn open_file_overwrite_rw<P: AsRef<Path>>(path: P) -> Result<File, DSEError> {
    let parent_path = path.as_ref().parent().ok_or(DSEError::Invalid("Path is not a file!".to_string()))?;
    std::fs::create_dir_all(parent_path)?;
    println!("[*] Opening file {:?} for rw", path.as_ref());
    let mut file = OpenOptions::new().append(false).create(true).read(true).write(true).open(path)?;
    file.set_len(0)?;
    file.seek(std::io::SeekFrom::Start(0))?;
    Ok(file)
}

pub fn valid_file_of_type<P: AsRef<Path>>(path: P, t: &str) -> bool {
    if let Ok(file_metadata) = std::fs::metadata(&path) {
        let is_file = file_metadata.is_file();
        let extension = path.as_ref().extension();
        if let Some(extension) = extension {
            if let Some(extension) = extension.to_str() {
                is_file && extension.to_lowercase() == t.to_lowercase()
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}

