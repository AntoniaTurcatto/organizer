use std::error::Error;
use std::fs::DirEntry;
use std::path::PathBuf;
use std::{fs, io};
use std::ffi::OsString;

use chrono::{DateTime, Datelike, Local};

use crate::filters::{DateFilter, Filter};

pub type RecoverableErrorVec = Vec<Box<dyn Error>>;

pub mod filters {
    pub enum Filter {
        Ext,
        Date(DateFilter),
    }

    pub enum DateFilter {
        Day,
        Month,
        Year,
    }
}

pub fn filter_folder(
    path: &str,
    filter: &filters::Filter,
) -> Result<Option<RecoverableErrorVec>, Box<dyn Error>> {
    if !fs::exists(path)? {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Diretório não encontrado: {}", path),
        )));
    }

    let folder_entities = fs::read_dir(path)?;
    let mut errors: Vec<Box<dyn Error>> = Vec::new();
    for entities_res in folder_entities {
        match entities_res {
            Err(e) => {
                errors.push(Box::new(e));
            }
            Ok(dir_entry) => {
                if let Err(e) = filter_dir_entry(dir_entry, filter) {
                    errors.push(e);
                }
            }
        }
    }
    if errors.is_empty(){
        Ok(None)
    } else {
        Ok(Some(errors))
    }
}

fn filter_dir_entry(dir_entry: DirEntry, filter: &Filter) -> Result<(), Box<dyn Error>> {
    if !dir_entry.file_type()?.is_file() {
        return Ok(());
    }
    let (orig_path, target_path) = get_parent_folder_and_target(&dir_entry, filter)?;

    if let Some(parent) = target_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    fs::rename(orig_path, target_path)?;
    Ok(())
}

fn get_parent_folder_and_target(dir_entry: &DirEntry, filter: &Filter) -> Result<(PathBuf, PathBuf), Box<dyn Error>>{
    let orig_path_binding = dir_entry.path();

    let target = match filter {
        Filter::Ext => {
            orig_path_binding
                .extension()
                .ok_or("Could not get file extension")?
                .to_os_string()
        }
        Filter::Date(date_filter)=>{
            let sys_time = orig_path_binding
                .metadata()?
                .modified()?;
            let date_time: DateTime<Local> = DateTime::from(sys_time);
            let target_num = match date_filter {
                DateFilter::Day   => date_time.day() as i32,
                DateFilter::Month => date_time.month() as i32,
                DateFilter::Year  => date_time.year(),
            };
            OsString::from(target_num.to_string())                
        }
    };

    let parent_folder = orig_path_binding
        .parent()
        .ok_or("Cound not get parent folder")?
        .to_path_buf();

    let target_folder = parent_folder.join(target);
    let target_path = target_folder.join(orig_path_binding.file_name().ok_or("File must have a name")?);
    Ok((orig_path_binding, target_path))
}