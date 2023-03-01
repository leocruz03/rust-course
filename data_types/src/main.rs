fn main() {
    /*
        Tipado en Rust

        El chqueo de los tipos se realiza durante la compilación, y no durante la ejecución. Comparado con el 
        tipado dinámico, el estático permite que los errores de tipificación sean detectados antes, y que la 
        ejecución del programa sea más eficiente y segura.
    */

    // *** Tipos de datos escalares *** //

    // integers

    /*
        pueden ser con signo o sin signo o sea si es negativa o positiva, y eso lo vamos a diferenciar con la 
        letra u o la letra i
    */

    // 8bit --> i8 (con signo) --> u8 (sin signo)
    // 16bit --> i16 (con signo) --> u16 (sin signo)
    // 32bit --> i32 (con signo) --> u32 (sin signo)
    // 64bit --> i64 (con signo) --> u64 (sin signo)
    // 128bit --> i128 (con signo) --> u128 (sin signo)
    // arch --> isize (con signo) --> usize (sin signo)

    /* 
        con el isize o usize el sistema automaticamente un valor dependiendo la arquitectura donde se esté 
        corriendo el software 
    */

    let num_one: i8 = -9;
    println!("Valor de entero con signo es: {}", num_one);

    let num_two: u32 = 78965;
    println!("Valor de entero sin signo es: {}", num_two);

    // # Enteros literales #

    // número decimal
    let decimal = 98_222;
    println!("{}", decimal);

    // número hexadecimal
    let hex = 0xff;
    println!("{}", hex);

    // número octal
    let octal = 0o77;
    println!("{}", octal);

    // número binario
    let binary = 0b1111_0000;
    println!("{}", binary);


    // float (números de coma flotante)

    /*
        solo hay dos tamaños en este tipo de dato, el f32 y el f64, y por defecto si no le asignamos el 
        tamaño a un valor de punto flotante la maquina le da el tamaño de f64, estos dos tamaños reciben 
        tanto negativos como positivos.
    */

    let float32: f32 = 3.257986;
    println!("Valor de la variable de tipo f32 es de: {}", float32);
    
    let float64_positive: f64 = 5676.899123;
    let float64_negative: f64 = -456.899123;
    println!("Valor de la variable de tipo f64 positive es de: {}", float64_positive);
    println!("Valor de la variable de tipo f64 negative es de: {}", float64_negative);

    // booleans

    /*
        este tipo de dato solo tiene dos posibles valores: true o false
    */

    let is_active: bool = true;
    println!("User is active? {}", is_active);

    let is_married: bool = false;
    println!("User is married? {}", is_married);

    // chars

    /*
        este tipo de dato ocupa cuatro 4 bits y recibe emojis, carácteres chinos, japoneses, etc...; en este 
        tipo de dato se usan las comillas simples 
    */

    let letter: char = 'b';
    println!("El valor de la variable de tipo char letter es de: {}", letter);
    
    let emoji: char = '🤨';
    println!("El valor de la variable de tipo char emoji es de: {}", emoji);

    // *** Tipos de datos compuestos *** //

    // tuples

    /*
        es una forma de agrupar valores de diferentes tipos de datos y tienen un valor fijo, o sea no se pueden 
        modificar
    */

    let my_tuple: (char, u8, i8, f32) = ('h', 23, -45, 0.2222);
    println!("El valor del elemento en la segunda posición en la tupla my_tuple es de: {}", my_tuple.1);

    // la tupla también se puede separar en diferentes variables

    let (w, x, y, z) = my_tuple;
    println!("El valor de 'y' el cual le pertenece a la tupla my_tuple, es: {}", y);

    /* 
        también para acceder a los valores de la tupla se hace referencia a las pocisiones y va organizado de 0 
        en adelante
    */

    println!("El segundo valor de la tupla my_tuple es: {}", my_tuple.1);

    // array

    /*
        los arrays en rust solo pueden ser de un solo tipo de dato y se les establece el tipo de dato que tiene y 
        la cantidad o longitud que poseé
    */

    let my_arr: [u64; 5] = [1, 2, 3, 4, 5];

    /* 
        para acceder a los valores de un array se hace como en cualquier otro lenguaje, por medio de los [] y adentro
        de el un número el cual debe ser una posición valida
    */

    println!("El valor del elemento en la terecera pocisión en el array my_arr es de: {}", my_arr[2]);

    // strings

    /*
        en rust existen dos tipos de strings, el static y que no puede crecer ni decrecer, pero el otro string si puede 
        crecer y decrecer
    */

    let name: &str = "John";
    println!("El valor de la variable name es de: {}", name);
    
    // el otro string
    let last_name: String = String::from("Doe");
    println!("El valor de la variable last_name es de: {}", last_name);
}
