fn main() {
    
    // Programación concurrente con Async

    /*
        el problema que teníamos anteriormente con los hilos de los sistemas operativos 
        es que el crearlas y ejecutar una acción en ellos, eso es muy costoso para el 
        computador, una de las soluciones que se implementan es Async, el cual lo puso de 
        moda Node JS

        viendo los beneficios de cada uno, ¿cuál elegimos ahora?
        
        pues esto depende de lo que queramos realizar o de como esté implementado nuestro 
        código 
    */

    
    /* let _content = file_reader("foo.txt").await;
    println!("{_content}") */
    
}

async fn file_reader(path: &str) -> String {
    println!("leyendo archivo: {path}...");

    // cómo leer un archivo puede ser muy pesado, es mejor ejecutarlo en background por eso se utiliza los del sistema operativo
    std::thread::sleep(std::time::Duration::from_secs(2));
    "file_name".to_string()
}
