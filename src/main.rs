use std::fs;
use std::env;
use std::path::Path;
use std::io;
extern crate walkdir;
use walkdir::WalkDir;
use std::ffi::OsStr;
fn main() {
	let args: Vec<String> = env::args().collect();
	let ext = &args[1];
	println!("Extension {}", ext);
	recorrer_Path(&ext);
}


fn recorrer_Path(ext: &str){
	let directory = Path::new(".");
	for paths in WalkDir::new(&directory) {
		let p= paths.unwrap();
		let file_name = p.file_name();
		let file_name_as_str = file_name.to_str().unwrap();
		if file_name_as_str.contains(ext) == true {
			println!("{:?}", p.path().display());
		};
		
	}
}