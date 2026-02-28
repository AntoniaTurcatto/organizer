use std::process;
use clap::{arg, command};
use organizer::filters::{DateFilter, Filter};

/// The main entry point for the file organizer CLI application.
///
/// This application parses command-line arguments to determine the directory
/// to organize and the criteria to use (extension or date-based).
fn main() {
    let (filter, path) = init();
    match organizer::filter_folder(&path, &filter) {
        Err(e) =>{
            eprintln!("Could not organize provided folder: {e}");
            process::exit(1);
        }
        Ok(recv_errors_op)=>{
            println!("Done!");
            if let Some(recv_errors) = recv_errors_op{
                println!("Obtained some errors:");
                for error in recv_errors{
                    println!("{error}");
                }
            }
        }
    }
}

/// Initializes the application by parsing command-line arguments using `clap`.
///
/// # Arguments
///
/// * `--folder <PATH>`: The path to the directory to be organized (Required).
/// * `--time <TIME-FILTER>`: Optional. Defines the date-based organization criteria.
///    * `d`: Organize by day of modification.
///    * `m`: Organize by month of modification.
///    * `y`: Organize by year of modification.
///    If omitted, files are organized by extension.
///
/// # Returns
///
/// A tuple containing the chosen [`Filter`] and the path to the folder as a `String`.
///
///#Panics
///Panics if the folder is not informed
fn init() -> (Filter, String) {
    const DAY: &str = "d";
    const MONTH: &str = "m";
    const YEAR: &str = "y";

    let matches = command!()
        .arg(arg!(--folder <PATH>).required(true))
        .arg(arg!(--time <"TIME-FILTER">).value_parser([DAY, MONTH, YEAR]))
        .get_matches();

    let filter = if let Some(time_spec) = matches.get_one::<String>("time") {
        if time_spec == DAY {
            Filter::Date(DateFilter::Day)
        } else if time_spec == MONTH {
            Filter::Date(DateFilter::Month)
        } else {
            Filter::Date(DateFilter::Year)
        }
    } else {
        Filter::Ext
    };

    (filter, matches.get_one::<String>("folder").unwrap().to_owned())
}
