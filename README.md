# organizer

 is a Rust library and CLI tool designed to help you organize files within a directory based on specific criteria, such as file extension or modification date. It efficiently moves files into dynamically created subdirectories.

## Features

- **Organize by Extension**: Group files together based on their type (e.g., all `.txt` files in a `txt` folder).
- **Organize by Date**: Group files based on their last modification time (Year, Month, or Day).
- **CLI Tool**: Ready-to-use command-line interface.
- **Robust Error Handling**: Non-fatal errors are collected and reported without stopping the entire process.

## Installation

### As a Library
Add this to your `Cargo.toml`:

```toml
[dependencies]
organizer = "0.1.0" # Check for the latest version on crates.io
```

Or run: 
```bash
    cargo add organizer
```

### As a CLI Tool
Install directly from crates.io:

```bash
cargo install organizer
```

## Usage

### Command Line Interface (CLI)

```bash
# Organize files by extension (Default)
organizer --folder /path/to/your/files

# Organize files by year of modification
organizer --folder /path/to/your/files --time y

# Organize files by month of modification
organizer --folder /path/to/your/files --time m

# Organize files by day of modification
organizer --folder /path/to/your/files --time d
```

### Library Usage

```rust
use organizer::{filter_folder, filters::{Filter, DateFilter}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let folder_path = "./my_files";
    
    // Define the filter criteria (e.g., organize by Year)
    let filter = Filter::Date(DateFilter::Year);

    // Apply the filter
    match filter_folder(folder_path, &filter) {
        Ok(None) => println!("Organization completed successfully!"),
        Ok(Some(errors)) => {
            println!("Organization completed with {} errors.", errors.len());
            for error in errors {
                eprintln!("Error: {}", error);
            }
        }
        Err(e) => eprintln!("Fatal error: {}", e),
    }

    Ok(())
}
```

## License

This project is licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
