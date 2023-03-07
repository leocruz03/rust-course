use std::{thread, time::Duration, sync::mpsc};

// comunicación entre hilos, para eso se usa el concepto Channels
fn main() {
    let name = String::from("Leonardo");

    let (tx, rx) = mpsc::channel(); // enviar mensaje a el hilo principal, rx = transmisor y tx = receptor

    // para crear un nuevo thread
    thread::spawn(move || {
        // no va a dejar que se ejecute
        println!("Hi, from thread one");
        println!("{}", name);
        
        // enviar varios mensajes
        for count in 0..3 {
            let mut msg = String::from("Counter: ");
            msg.push(char::from_digit(count, 10).unwrap());
            // multiples productores de mensajes pero solo un consumidor
            let msg = String::from("Hello");
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    println!("Hi, from main thread");

    // recibir el mensaje
    let msg_recived = rx.recv().unwrap();
    println!("message recived: {msg_recived}");

    /* 
        // el joinHandle nos permite controlar cuando un hilo espera hasta que termine el hilo que está esperando
        join_handle.join().unwrap(); // el unwrap, es porque join devuelve un result 
    */

    // recibir varios mensajes
    for msgs_recived in rx {
        println!("mensaje recibido: {msg_recived}")
    }

}
