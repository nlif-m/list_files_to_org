#![warn(clippy::all, clippy::pedantic)]
use std::fs;
use std::env;
use std::cmp::Ordering;
// TODO: intro clap crate
// TODO: add a output flag
fn files_to_org(it: fs::ReadDir, level: u16) {
    for entry in it {
        let mut dirs: Vec<fs::DirEntry> = Vec::new();
        let entry = entry.unwrap();
        let name = entry.file_name();
        let file_type = entry.file_type().unwrap();
	let level_str = "*".repeat(level.into());
        if file_type.is_file() {
            println!("{level_str} {:?}", name);
        } else if file_type.is_dir() {
            dirs.push(entry);
        }

	for dir in dirs {
	    let path = dir.path();
	    let name = dir.file_name();
            println!("{level_str} {:?}", name);
	    files_to_org(fs::read_dir(path).unwrap(), level + 1);
	}
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len().cmp(&2) {
	Ordering::Less | Ordering::Greater  => {
	    println!("ERROR: provide a one dir path");
	    std::process::exit(1);
	},
	// Ordering::Greater => {
	//     println!("ERROR: provide a one dir path");
	//     std::process::exit(1);
	// },
	Ordering::Equal => {
	    let dir = fs::read_dir(&args[1]).unwrap();
	    files_to_org(dir, 1);
	}
    }
}
