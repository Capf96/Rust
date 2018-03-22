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
		let k = Path::new(file_name);
		if file_name_as_str.contains(ext) == true {
			println!("{:?}", p.path().display());
		};
		
 

		/*let c = match k.extension().and_then(OsStr::to_str){
			Some(x) => if x == ext { Some(path.display()) } else { None },
			None => None,
		};
		if c != None { 
			match c {
				Some(x) => println!("{}", x),
				_ => println!("Esto nunca se va a imprimir"),
			}
		}*/
	
		
	}
}