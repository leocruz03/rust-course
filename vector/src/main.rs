fn main() {
    
    // Vector

    /*
        el vector permite guardar valores, uno a uno al lado del otro en la memoria, pero todos tienen 
        que ser del mismo tipo de dato
        
        para inicializar vectores, ya que son tan usados, rust nos provee de un método auxiliar la cual 
        es utilizando una macro
    */

    // inicializar vectores
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3, 4, 5];

    /*
        para poder añadir, eliminar y otros procesos similares en un vector se necesita establecer que 
        el vector sea mutable
    */

    // igresar un nuevo dato al vector, solo se pueden ingresar datos del tipo de dato ya establecido
    v.push(6);

    // para acceder a los datos de un vector
    println!("el tercer elemento del vector 'v' es: {}", v[2]);
    // otra forma de hacerlo, más seguro y recomendado es
    match v.get(1) {
        Some(value) => println!("el segundo elemento del vector 'v' es: {}", value),
        _ => ()
    };

    // recorrer o acceder a todos los valores de un vector
    for i in &v {
        println!("value: {}", i)
    }

    // recorrer o acceder a todos los valores de un vector y hacer modificaciones
    for i in &mut v {
        *i += 10 // aquí se desreferencia
    };

    // crear un vector con diferentes tipos de datos
    #[derive(Debug)]
    enum Message {
        TEXT(String),
        ERROR(i32)
    }

    let msgs :Vec<Message> = vec![Message::TEXT(String::from("Hellooo")), Message::ERROR(404)];
    println!("{:?}", msgs);

    for msg in &msgs {
        match msg {
            Message::TEXT(text) => println!("message: {}", text),
            Message::ERROR(error_code) => println!("error code: {}", error_code),
        }
    }
}
