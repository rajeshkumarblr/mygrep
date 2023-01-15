extern crate clap;
use clap::{App, Arg};
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let matches = App::new("My Grep")
        .version("1.0")
        .author("Rajesh Kumar")
        .about("Searches recursively for a string in a directory")
        .arg(
            Arg::with_name("directory")
                .short("d")
                .long("directory")
                .value_name("DIR")
                .help("Sets the directory to search")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("pattern")
                .short("p")
                .long("pattern")
                .value_name("PATTERN")
                .help("Sets the pattern to search for")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let directory = matches.value_of("directory").unwrap();
    let search_string = matches.value_of("pattern").unwrap();

    search_directory(directory, search_string);
}

fn search_directory(dir: &str, search_string: &str) {
    let path = Path::new(dir);
    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let entry_path = entry.path();
            if entry_path.is_dir() {
                search_directory(entry_path.to_str().unwrap(), search_string);
            } else {
                search_file(&entry_path, &search_string);
            }
        }
    } else {
        search_file(path, search_string);
    }
}

fn search_file(file_path: &Path, search_string: &str) {
    let file = match fs::File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    let reader = BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        if let Ok(line) = line {
            if line.contains(search_string) {
                println!("{}:{}:{}", file_path.display(), index + 1, line);
            }
        }
    }
}
