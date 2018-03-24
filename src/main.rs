use std::env;
use std::path::Path;
extern crate walkdir;
use walkdir::WalkDir;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::io::{BufReader,BufRead};


fn main() {
	/*Primero debo leer el archivo txt a ver si la busqueda 
	ya la he realizado, de lo contrario recorro el path y 
	almaceno en el txt*/
	let args: Vec<String> = env::args().collect();
	let ext = &args[1];
	println!("Extension {}", ext);
	leer(ext);
	escribir(recorrer_path(), ext);
	
	
}

fn escribir(vec: HashMap<String,String>, ext: &str){
	/*Recorro el hash map a ver si consigo el archivo*/
	for (key,value) in &vec{
		println!("{:?} -> {:?}",key, value);
		if key.contains(&ext.to_string()){
			println!("{} : {:?}", &ext.to_string(),value);
		}
	}
	/*Almaceno lo que esta en el hash map en un txt para una proxima busqueda*/
    let path = Path::new("busqueda.txt");
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("No se pudo crear el archivo {}: {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };
    for (key,value) in vec{
    	//Archivo path
    	let string = format!("{} {} \n", key, value);
    	match file.write_all(string.as_bytes()) {
	        Err(why) => {
	            panic!("No se pudo crear el archivo {}: {}", display, why.description())
	        },
	        Ok(_) => continue,
   		}	
    }
    
}

fn leer(ext: &str) {
	/*Leo el archivo como primera opcion para ver si 
	lo que busco se encuentra en el */
	let file = File::open("busqueda.txt");
	let archivo = match file {
		Ok(x) => x,
		Err(_) => return,
	};
	for line in BufReader::new(archivo).lines() {
		if line.unwrap().contains(ext){
			println!("Esta en el archivo pero no puedo imprimirlo :c");
			//println!("{:?}",line.unwrap());
		}
	}
}

fn recorrer_path() -> HashMap<String,String>{
	let directory = Path::new(".");
	let mut dir = HashMap::new();
	for paths in WalkDir::new(&directory) {
		let p= paths.unwrap();
		let file_name = p.file_name();
		let file_name_as_str = file_name.to_str().unwrap();
		if !p.file_type().is_dir() {
			/*Almaceno en el hash el nombre del archivo que consiga que no sea
			directorio junto a su ruta de busqueda.*/
			dir.insert(String::from(file_name_as_str),p.path().display().to_string());
		};		
	}
	dir
}

