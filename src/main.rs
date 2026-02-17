use clap::command;
use clap::arg;

fn main() {
    let matches = command!()
        .arg(arg!(--filter <TYPE>).required(true))
        .arg(arg!(--time <"TIME-FILTER">).value_parser(["d", "m", "y"]))
        .get_matches();

    let filter_type = matches.get_one::<String>("filter").expect("filter expected.");
    //TODO: build Filter

}
