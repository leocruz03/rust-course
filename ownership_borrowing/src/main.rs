fn main() {
    // Ownership y Borrowing

    /* 
        ownership, significa propiedad (que es dueño de algo) y borrowing pedir prestado 
    */

    let name = String::from("John");
    let name_copy = name;
    println!("{}", name_copy); // es correcto ya que ahora name_copy es dueña de name
    // println!("{}", name); // es incorrecto, ya que name se le asignó a otra variable y ya no
    // existe por si sola

    /*
        esto se refiere al manejo de la memoria, ya que en rust no hay "garbage collector" 
        (recolector de basura), en otros lenguajes de programación varias variables pueden 
        apuntar al mismo espacio de memoria, pero en rust cada data tiene un dueño, solo 
        una variable puede apuntar a un espacio de memoria
    */

    /*
        este ejemplo es muy claro, tenemos una variable llamada name, la cual tiene un String de valor 
        en ese momento name era dueño de ese String, pero una línea de código después la variable 
        copy_name toma a name como su valor, o sea se apropia de valor de name y ahora pasa a ser de 
        copy_name

        let name = String::from("John");
        let name_copy = name;

        println!("{}", name); ❌
        println!("{}", copy_name); ✅
    */

    // dos conceptos super importantes vienen acompañados con: 

    // Stack y Heap

    /*
        el stack (pila) es una estructura de datos que se basa en la regla, "el último que entra 
        es el primero en salir", esta es una estructura de datos dinámica que permite almacenar y 
        recuperar datos, sus dos operaciones básicas son, push, agrega un elemento en la cola o 
        de último y el método pop, el cual elimina o retira el último elemento de la lista y en el 
        stack es liberada cuando se alcanza el fin del scope. 
    */

    /*
        el heap (montículo) es una estructura de datos bastante flexible (puede crecer, decrecer,...), 
        tiene más costos de memoria y también al momento de recuperar datos, etsa memoria heap es libe-
        rada cuando no tiene más dueños
    */
}
