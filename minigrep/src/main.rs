/*
    --> env: libreria para recibir los parámetros recibidos por consola 
    --> fs: para abrir archivos y poder hacer cosas con ellos
*/
use std::{env};

use minigrep::Config;

fn main() {
    // Primer programa en rust

    /*
        se va a desarrollar un programa en línea de comandos el cual buscará por referencia 
        las palabras iguales a la especificada
    */

    let args: Vec<String> = env::args().collect(); 

    /*
        se van a recibir los argumentos a traves del método args(), esto nos devuelve un iterator 
        y con el método collect() volvemos estos valores una colección, entonces la variable será 
        un vector de tipo String
    */

    println!("{:?}", args);

    let config = Config::new(&args);
    
    println!("nombre del archivo: {}", config.filename);
    println!("string a buscar: {}", config.query);

    minigrep::run(config)
}