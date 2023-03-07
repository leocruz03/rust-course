use std::{sync::Mutex, sync::Arc};

fn main() {
    
    // Accediendo desde múltiples threads al mismo dato

    /*
        mutexes es el concepto que permite el acceso al mismo dato desde distintos hilos, por
        turnos, esto crea un mutex el cual contiene un valor: Mutex(value), y también está el 
        lock que se usa para cuando un hilo quiera usar el dato, que se podría decir que es un 
        bloqueador, o sea que si aplicamos el lock a mutex, se podría decir que está bloqueado
    */

    /*
        Arc, es similar a Rc, pero es thread safe, es seguro de usarlo en situaciones concurrentes.

        Arc: Atomic Reference Counted (smart pointer)
        atomic: son primitivos seguros de compartir en distintos hilos
    */

    let id = Arc::new(Mutex::new(99));
    let mut handles = vec![];

    for _ in 0..3 {
        let num_clone = Arc::clone(&id);
        let handle = std::thread::spawn(move || {
            let mut num = num_clone.lock().unwrap();
            *num += 1
        });
        handles.push(handle);

    }
    
    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", id);
}
