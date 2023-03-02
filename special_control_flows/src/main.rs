fn main() {
    // if let
    let age: Option<i32> = Some(20);

    /*  
        en el if let, es un patrón y un valor y si son iguales entra al bloque de código
    */

    if let Some(value) = age {
        println!("edad: {}", value);
    }

    // while let

    /* 
        su objetivo es hacer más conciso el código
    */

    let mut msgs_noread = Some(100);

    while let Some(value) = msgs_noread {
        if value > 0 {
            println!("tienes mensajes no leídos");
            msgs_noread = Some(value - 1);
        } else {
            println!("no hay mensajes nuevos");
            msgs_noread = None;
        }
    }

    // let else

    let some_number: Option<i32> = Some(100);

    let Some(number) = some_number else {
        panic!("el número no es valido");
    };
    println!("número valido {}", number);


}
