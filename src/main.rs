use ::std::process;
use clap::{arg, command};
use organizer::filters::{DateFilter, Filter};

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
