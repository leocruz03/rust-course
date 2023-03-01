fn main() {
    /*
        Tipado en Rust

        El chqueo de los tipos se realiza durante la compilación, y no durante la ejecución. Comparado con el 
        tipado dinámico, el estático permite que los errores de tipificación sean detectados antes, y que la 
        ejecución del programa sea más eficiente y segura.
    */

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
    println!("{}", binary)
}
