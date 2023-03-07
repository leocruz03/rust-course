// librería para crear un hilo y para comunicar los hilos se utiliza la librería std::sync::mpsc
use std::{thread, time::Duration, sync::mpsc, char};


fn main() {
    
    // Comunicación entre threads
    
    /*
    para la comunicación etre hilos se usa el concepto de channels, el cual sirve para mandar 
    mensajes entre los hilos
    */

    /*
        el mpsc, significa multiple producer single consumer (varios productores un solo consumidor)
    */

    // tx, es el transmisor y el rx es el receptor
    let (tx, rx) = mpsc::channel();
    let tx_two = tx.clone();

    let name: String = String::from("leonardo");
    println!("está en el hilo main: {name}");
    
    // hilo uno
    thread::spawn( move || {

        println!("entró al hilo uno: {name}");
        println!("Hola desde el hilo 1");
        
        // enviar mensaje hacía el hilo main o principal
        for count in 0..3 {
            let mut msg: String = String::from("Mensaje desde el hilo uno hacía el hilo main \n");
            msg.push(char::from_digit(count, 10).unwrap());
            tx.send(msg).unwrap();
            println!("contador: {count}");
            thread::sleep(Duration::from_secs(2));
        }
    });

    
    // hilo dos
    thread::spawn( move || {
        for count in ['a', 'b', 'c', 'd', 'e'].iter() {
            let mut msg: String = String::from("letra: ");
            msg.push(*count);
            tx_two.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    // recibir mensajes de los hilos uni y dos
    for msg_recived in rx {
        println!("el mensaje dede el hilo 1 hasta el hilo main fue recibido: {msg_recived}");
    }

    println!("Hola desde el hilo principal");
}
