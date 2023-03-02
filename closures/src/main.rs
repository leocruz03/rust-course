fn main() {
    // Closures

    /* 
        Algunas veces es útil envolver una función y sus variables libres para mejor claridad y reusabilidad. Las 
        variables libres que pueden ser usadas provienen del ámbito exterior y son "cerradas" cuando son usadas en 
        la función. De allí el nombre closure. Rust provee una muy buena implementación, como veremos

        let suma_uno = |x: i32| x + 1;
        suma_uno(1) // = 2

        creamos un enlace a variable, suma_uno y lo asignamos a un closure. Los argumentos de closure van entre pipes 
        "|", y el cuerpo es una expresión, de manera que podemos tener closures multi-tarea
    */

    let sum = |nro_uno: i32, nro_dos: i32| {
        nro_uno + nro_dos
    };

    println!("{}", sum(4, 2));


    let counter = 1;

    let incrementar = || {
        println!("{}", counter);
    };

}
