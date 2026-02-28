use std::error::Error;
use std::fs::DirEntry;
use std::path::PathBuf;
use std::{fs, io};
use std::ffi::OsString;

use chrono::{DateTime, Datelike, Local};

use crate::filters::{DateFilter, Filter};

/// A type alias for a vector of boxed errors that can be returned when
/// processing multiple files, allowing the process to continue despite individual failures.
pub type RecoverableErrorVec = Vec<Box<dyn Error>>;

/// Contains types and definitions for filtering and organizing files.
pub mod filters {
    /// Defines the criteria used to organize files.
    pub enum Filter {
        /// Organizes files based on their file extension.
        Ext,
        /// Organizes files based on their modification date, using a specific precision.
        Date(DateFilter),
    }

    /// Defines the precision level for date-based organization.
    pub enum DateFilter {
        /// Organizes files into folders named after the day of the month (e.g., "15").
        Day,
        /// Organizes files into folders named after the month number (e.g., "3").
        Month,
        /// Organizes files into folders named after the year number (e.g., "2025").
        Year,
    }
}

/// Scans a directory and organizes its files based on the provided [`Filter`].
///
/// Files are moved into subdirectories named after the criteria calculated from the filter.
///
/// # Errors
///
/// Returns an error if:
/// * The provided `path` is not a valid directory.
/// * There are issues reading the directory entries.
/// * A fatal error occurs during file movement.
///
/// If some files fail to move but others succeed, this function returns `Ok(Some(RecoverableErrorVec))`
/// containing the errors encountered for individual files.
///
/// # Examples
///
/// ```rust
/// use organizer::{filter_folder, filters::{Filter, DateFilter}};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let directory = "./target_folder";
/// // Create directory for testing
/// std::fs::create_dir_all(directory)?;
///
/// let filter = Filter::Date(DateFilter::Year);
///
/// match filter_folder(directory, &filter) {
///     Ok(None) => println!("All files organized successfully!"),
///     Ok(Some(errors)) => println!("Organized with {} errors.", errors.len()),
///     Err(e) => eprintln!("Fatal error: {}", e),
/// }
/// # Ok(())
/// # }
/// ```
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

/// Processes a single directory entry, applying the filter and moving the file if necessary.
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

/// Calculates the original path and the target path for a given file entry based on the filter.
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