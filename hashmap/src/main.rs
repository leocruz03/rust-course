use std::collections::HashMap;

fn main() {
    
    // Hashmap

    /*
        esta es una estructura de datos que permite guardar valores y asociarlos a una key o llave 
        para acceder a ellos, los maps en rust se guardan en la memoria heap para que puedan ser 
        modificables y todas la llaves tienen que ser del mismo tipo y todos los datos también. No está 
        incluido en el preludio entonces uno tiene que importarlo desde la librería estandar.
    */

    // inicializar un hashmap
    let mut scores: HashMap<String, i32> = HashMap::new();

    // insertar datos, para modificar se necesita que sea mutable
    scores.insert("Blue".to_string(), 20);
    scores.insert("Red".to_string(), 100);

    // obtener los datos
    let blue_score = scores.get("Blue");
    match blue_score {
        Some(score) => println!("the score of team blue is: {}", score),
        _ => println!("team blue don't has score"),
    }
    // println!("the score of team blue is: {}", blue_score.unwrap())

    /*
        cuando ya tenemos una llave igual a la que queremos ingresar pero con un valor diferente 
        se sobreescribe el valor en rust
    */
    scores.insert("Blue".to_string(), 34);

    // iterar un hashmap
    for (k, v) in &scores {
        println!("{}, {}", k, v)
    }
    
    // no se sobreescribe, solo si ve que la key no existe se crea
    scores.entry("Green".to_string()).or_insert(20); 
    for (k, v) in &scores {
        println!("{}, {}", k, v)
    }
    
}
