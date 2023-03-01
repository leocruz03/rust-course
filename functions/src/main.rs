fn main() {
    // Funciones

    /* 
        las funciones, son trozos de código el cual se puede reutilizar para que el código no sé vuelva 
        repetitivo. Sobre este tema se creo el paradigma funcional, las funciones llegaron a evitar el 
        código spaguethi y lo hace ver más organizado y estetico. 
    */

    /* 
        en rust la sintaxis de una función se empieza con la palabra reservada "fn", el nombre de la 
        función, el espacio para los parámetros y dentro de las llaves el código que ejecuta esa 
        función, en rust hay una función principal, la cual se encargará de ejecutar el programa y la 
        regla de la sintáxis del nombre de la función debe ser "snake_case".
    */

    show_welcome();
    select_number(6);

    // cómo retorna un valor, este valor se guarda en una variable y puede utilizar para otras cosas
    let person_one = can_enter(true);
    println!("El estado de la persona uno para entrar es: {}", person_one);

    hi_with_name("John".to_string());

    let mut last_name: String = String::from("doe");

    /*
        - fn nombre_de_funcion(variable: String) toma un String y se hace dueño de él.
        - fn nombre_de_funcion(variable: &String) toma prestado un String y puede acceder a su valor.
        - fn nombre_de_funcion(variable: &mut String) toma prestado un String, puede acceder a su valor y modificarlo.
    */

    reference_parameters(&last_name);
    add_name(&mut last_name)
}

// sintáxis básica
fn show_welcome() {
    println!("Bienvenidos a Rust");
}

// función que recibe parámetros, a los parámetros siempre se les indica el tipo de dato
fn select_number(nro: i32) {
    println!("Su número es: {}", nro);
}

// función con retorno, la cual se le debe especificar el tipo de dato que retorna
fn can_enter(can: bool) -> bool {
    return can;
}

// pasar un string por parámetro
fn hi_with_name(name: String) {
    println!("Hola: {}", name)
}

// pasar parámetros por referencia
fn reference_parameters(last_name: &String) {
    println!("El apellido de la persona es: {}", last_name);
}

// pasar mutabilidad por referencia
fn add_name(last_name: &mut String) {
    last_name.push_str(" - john");
}
