// librería para crear un hilo

use std::{thread, time::Duration};


fn main() {

    // Concurrency

    /*
        la concurrencia es la capacidad de un sistema para procesar más de un hilo en ejecución 
        al mismo tiempo.

        ¿Son necesarios varios procesadores?
        No. La concurrenciasi puede darse en un sistema mono - procesador. Hay concurrencia si 
        los procesos "conviven" en el mismo instnte de tiempo
    */
    
    /*
        spawn es el método que nos permite crear un hilo, al cual se le debe pasar una closure 
        y dentro de ese closure se ejecuta el código que va a estar en el nuevo hilo, ya ese 
        viene siendo un scope diferente
    */

    /*
        crear una variable en un hilo y usarlo en otro, eso lo podemos hacer con la palabra reservada 
        move, con esta palabra podemos hacer que las variables del scope principal se muevan hacía el 
        scope del otro hilo, al cual se le está pasando la palabra reservada, pero después de ejecutar 
        el hilo uno, ya la variable no existe como se cambio de scope, en el scope principal se borró.

        una de las formas para no perder el scope de la variable es usando clonandola, pero lo más po- 
        sible es que a la larga en un software que piensa ser escalable no es recomendado hacerlo 
        aunque eso siempre depende del desarrollador que haga el software.
    */
    let name: String = String::from("leonardo");
    println!("en el hilo principal: {name}");

    // el joinHandle es el que se encarga de esperar a que el hilo termine de ejecutar para que inicie el otro
    let join_handle = thread::spawn( move || {
        println!("en el hilo uno: {name}");
        thread::sleep(Duration::from_millis(2000));
        println!("Hola desde el hilo 1");
    });
    
    // se pone un sleep, el cual duerme al hilo que ejecuta el código
    // thread::sleep(Duration::from_millis(1000));
    println!("Hola desde el hilo principal");

    // aquí decimos que cuando se ejecute el hilo main, se ejecute el hilo uno
    join_handle.join().unwrap();

    /*
        antes del sleep, primero mostraba el resultado del hilo main y después el del hilo 1 
        pero ahora si ponemos sleep sobre la acción que hace main, primero se ejecuta el hilo 1 
        y después el hilo main, ya que se establecio que se durmiera por un segundo.

        pero si lo ponemos al revés donde el sleep estuviese sobre la acción del nuevo hilo, 
        recién creado, no se ejecuta nunca, porque como al momento de ejecutar el programa el 
        hilo principal ve que sobre el hilo 1 se duerme, se sigue ejecutando y cuando llega 
        a donde esta su acción, la ejecuta y hay acaba la ejecución del programa.

        No se ejecuto porque el hilo uno, el tiempo que se quedo dormido fue mayor que el 
        tiempo que se demoró ejecutando el hilo principal o el main.
    */

    println!("Ambos hilos se ejecutaron");
}   
