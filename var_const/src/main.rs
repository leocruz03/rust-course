const GLOBAL_CONST: &str = "Soy una constante global";

fn main() {

    /*
        Creación de variables en Rust

        las variables se crean con la palabra reservada let, después se le da el nombre a la variable, 
        se le asigna un valor o se puede declarar y después asignarle el valor
    */

    let my_var = 8; // variable con valor adjunto

    let my_var_two; // variable declarada
    my_var_two = 4; // después se le asigno el valor
    println!("El valor de la variable my_var_two es: {}", my_var_two);

    /*
        Inmutabilidad en Rust

        las variables por defecto en rust son inmutables, o sea que después de que les establezca el valor 
        no puede cambiar, entonces para poder añadir la funcionabilidad de la mutabilidad se le añade la 
        palabra reservada "mut"
    */

    let num = 10;
    // num = 8; ❌
    println!("El valor de la variable inmutable num es: {}", num);

    let mut num_two = 4;
    println!("El valor de la variable num_two es: {}", num_two);

    num_two = 3; // ✔
    println!("El nuevo valor de la variable num_two es: {}", num_two);

    /*
        Shadowing en Rust

        es cuando se crea una variable que tiene un valor y que en un momento preciso se le cambia el 
        valor, es como si sobreescribiera la variable o también se puede utilizar cuando se quiere hacer 
        una operación con la misma variable, o si hay una función entre medio.
    */

    let shadow_var = 7;
    println!("El valor de shadow_var es: {}", shadow_var);

    // ... cualquier proceso

    let shadow_var = 5;
    println!("Hice el proceso de shadowing con shadow_var y su valor es: {}", shadow_var);

    /*
        Constantes en Rust

        en rust las constantes se definen con la palabra reservada const, pero hay reglas al momento de 
        usar o crear constantes, el nombre de las constantes siempre debe ser en mayúsculas y siempre se 
        le debe establecer el tipo de dato, y una cosa también importante es que las constantes no pueden 
        utilizar la palabra reservada mut y tampoco se puede hacer shadowing con ellas; otro dato importante 
        es que las constantes no tienen scope predefinido, o sea se pueden utilizar y definír desde cualquier 
        scope.
    */

    const PI: f32 = 3.14159;
    println!("El valor de constante PI es: {}", PI);

    // imprimiendo constante global
    println!("Imprimendo una constante global con valor de: {}", GLOBAL_CONST);

    /*
        Mostrar variables o concatenar y mostrar en consola

        se utiliza {}, para concatenar variables a strings y mostrarlos en consola
    */
    println!("Mi primera variable tiene de valor: {}", my_var);
}
