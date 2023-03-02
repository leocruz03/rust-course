use std::ops::RangeTo;

fn main() {
    
    // Flujos de control

    /*
        los flujos de control son los que nos permiten tener más control sobre los procesos que se realiza 
        en nuestro código, a veces necesitamos repetir cierta acción, una cierta cantidad de veces; a veces 
        también vamos a necesitar manejar permisos y para eso podemos utilizar los flujos de control
    */

    // condicionales if ...else if ...else
    let age = 40;

    if age < 18 {
        println!("your age not enough for enter")
    } else if age >= 18 && age <= 25 {
        println!("'u can enter")
    } else {
        println!("was a older for enter")
    }

    // forma corta de las condicionales
    let num = 5;

    let resultant = if num >= 5 { 1000 } else { 0 };

    println!("resultado: {}", resultant);

    // --------------------------------------------------------------------------------------------------------- //

    // loop
    
    /* 
        con loop se puede ejecutar infinitamente un código hasta que uno lo detenga
    */

    let mut counter = 0;

    loop {
        counter += 1;

        println!("loop");
        if counter == 5 {
            println!("El contador llego hasta: {}", counter);
            break;
        }
    }

    // while

    /*
        este es un bucle el cual solo parará de ejecutarse hasta que se cumpla la condición definida
    */

    while counter > 0 {
        counter -= 1;
        println!("counter: {}", counter);
    }

    // for

    let arr = [1, 2, 3, 4, 5];

    for i in arr.iter() {
        println!("i: {}", i);
    }

}
