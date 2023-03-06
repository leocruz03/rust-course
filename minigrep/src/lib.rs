// este archivo contiene toda la lógica que no debe estar en el main

use std::fs;

// creación de configuración en un Struct
pub struct Config {
    pub filename: String,
    pub query: String,
}

// se implementa un método para inicializar la Struct
impl Config {
    pub fn new(args: &Vec<String>) -> Config {
        /*
            fragmentar el vector en varias variables
        */
        let filename = args[1].clone(); // se está pasando la referencia el segundo argumento del vector 
        let query = args[2].clone(); // se está pasando por referencia el tercer elemento del vector
    
        Config {filename, query}
    }
}

/*
    se usa el modificador pub, para que este archivo sea visibles para los otros
*/

pub fn run(config: Config) {
    // uso de la libreria para abrir un archivo externo, lee el nombre de un archivo y lo pasa a string
    let content = fs::read_to_string(config.filename).expect("Can't read to file");
    let found = search(&config.query, &content);

    for element in found {
        println!("{}", element);
    }
}

// indicar el tiempo de vida de las variables
pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {

    // variable para guardar los resultados
    let mut result = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}