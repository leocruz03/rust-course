use std::{fs::File};

fn main() {

    // Errors management --- gestión o manejo de errores

    /*
        en rust se identifican dos tipos de errores, los recuperables y los no recuperables; un ejemplo 
        de un error recuperable es abrir un archivo donde el path es incorrecto, y un ejemplo de un 
        error no recuperable, es tratar de acceder a un arreglo más allá de su limite
    */ 

    /*
        para los errores recuperables rust no tiene excepciones, en rust existe la estructura "Result<T, E>", 
        pero los no recuperables, rust tiene una macro para ellos, llamada "panic!"
    */

    // recuperable
    let file = File::open("some/errorpath");
    match file {
        Ok(file_ok) => read_file(file_ok),
        Err(error_file) => println!("the path doesn't exist: {}", error_file)
    }
}

fn read_file(file: File) {

}
