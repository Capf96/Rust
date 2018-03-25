use std::env;
use std::path::Path;
extern crate walkdir;
use walkdir::WalkDir;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::io::{BufReader,BufRead};
//use std::fs::OpenOptions;

/*----------------------------------------------------------------------------
									Main
----------------------------------------------------------------------------*/

fn main() {
	/*Primero debo leer el archivo txt a ver si la busqueda 
	ya la he realizado, de lo contrario recorro el path y 
	almaceno en el txt*/
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 || (args.len() == 3 && args[2] != "i") || args.len() > 3 {
		panic!("Argumentos incorrectos");
	}
	let ext = &args[1];
	let revisar = if args.len() == 3 && &args[2] == "i" { false } else { true };
	println!("Extension {}", ext);
	let lectura_archivo = leer();
	let lectura_archivo = if revisar {
		let temporal = recorrer_path(lectura_archivo);
		escribir(temporal)
	}
	else {lectura_archivo};
	busqueda(lectura_archivo, ext);
}

/*----------------------------------------------------------------------------
								Funciones
----------------------------------------------------------------------------*/

/*
Funcion busqueda(vec: HashMap<String,String>, ext: &str)

Parametros:
	vec: HashMap<String,String>	||	Tabla de hash que contiene los paths a 	
									los archivos en el arbol de directorios
									o el indice (depende de la opcion de 
									corrida)
	ext: &str 					||	String que contiene la coincidencia a 
									encontrar

Esta funcion busca dentro de la tabla de hash para encontrar si alguno de los 
archivos contiene la coincidencia a encontrar dentro del nombre del archivo
*/

fn busqueda(vec: HashMap<String,String>, ext: &str){
	/*Recorro el hash map a ver si consigo el archivo*/
	for (key,value) in vec{
		if value.contains(&ext.to_string()){
			println!("{} : {:?}", &ext.to_string(),key);
		}
	}
}

/*
Funcion escribir(vec: HashMap<String,String>) -> HashMap<String,String>

Parametros:
	vec: HashMap<String,String>	||	Tabla de hash que contiene los paths a 	
									los archivos en el arbol de directorios

Esta funcion escribe el archivo con todos los paths encontrados en el 
recorrido de los directorios realizado desde el directorio actual de corrida.
Los path son escritos en el archivo de la siguiente manera
direccion al archivo$nombre del archivo
La funcion devuelve una tabla de hash que contiene los paths y los nombres de
archivos.
*/

fn escribir(vec: HashMap<String,String>) -> HashMap<String,String> {
	
	/*Almaceno lo que esta en el hash map en un txt para una proxima busqueda*/
	
	//let mut recorrer_archivo = OpenOptions::new().write(true).append(true).open("busqueda.txt").unwrap();
	let path = Path::new("busqueda.txt");
	let display = path.display();
	let mut file = File::create(&path).expect("No se pudo crear el archivo");
	for (key,value) in &vec{
		//Almacena en el txt de la forma : path$nombreDelArchivo
		let string = format!("{}${}\n", key, value);
		match file.write_all(string.as_bytes()) {
			Err(why) => {
				panic!("No se pudo escribir en el archivo {}: {}", display, why.description())
			},
			Ok(_) => continue,
		}	
		/*let string = key+"$"+&value;
		if let Err(e) = writeln!(recorrer_archivo, "{}",string) {
			eprintln!("No se pudo escribir en el archivo: {}", e);
		}*/
	}
	vec
	
}

/*
Funcion leer() -> HashMap<String,String>

Parametros:
	None

Esta funcion lee el archivo indice llamado busqueda.txt y en caso de no 
existir, lo crea para su lectura o actualizacion posteriormente. La funcion
devuelve una tabla de hash con los paths encontrados dentro del archivo.
*/

fn leer() -> HashMap<String,String>  {
	/*Leo el archivo como primera opcion para ver si 
	lo que busco se encuentra en el */
	let mut dir = HashMap::new();
	let file = File::open("busqueda.txt");
	let archivo = match file {
		Ok(x) => x,
		Err(_) => {
			File::create("busqueda.txt").expect("Problema al crear archivo");
			let temporal = File::open("busqueda.txt");
			temporal.unwrap()
		}
	};
	for line in BufReader::new(archivo).lines() {
		let line = line.unwrap();
		let valor = line.rfind("$");
		let (first, second) = line.split_at(valor.unwrap());
		let second = second.trim_matches('$');
		dir.insert(String::from(first),String::from(second));
	}
	dir

}

/*
Funcion recorrer_path(mut dir: HashMap<String,String>) -> HashMap<String,String>

Parametros:
	dir: HashMap<String,String>	||	Tabla de hash que contiene los paths a 	
									los archivos en el indice el cual sera
									actualizado.

Esta funcion recorre el arbol de directorios y son agregados en la tabla de 
hash para su posterior escritura en el archivo y busqueda de coincidencia.
*/

fn recorrer_path(mut dir: HashMap<String,String>) -> HashMap<String,String>{
	let directory = Path::new(".");
	for paths in WalkDir::new(&directory) {
		let p = paths.unwrap();
		let file_name = p.file_name();
		let file_name_as_str = file_name.to_str().unwrap();
		if !p.file_type().is_dir() {
			/*Almaceno en el hash el nombre del archivo que consiga que no sea
			directorio junto a su ruta de busqueda. El hash por defecto si la clave
			o el directorio ya se encuentra en el hash, no lo inserta, lo descarta.*/
			dir.insert(p.path().display().to_string(), String::from(file_name_as_str));
		};		
	}
	dir
}

