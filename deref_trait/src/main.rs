fn main() {
    // Deref trait

    /*
        hace posible la dereferenciación (*)
    */

    let x = 5;
    let y = &x;

    if x == 5 {
        println!("{}", x)
    }

    if *y == 5 {
        println!("{y}")
    } 
    
    /* 
        da error porque no se puede comparar una referencia de entero, con un entero, para poder compararlo 
        se usa el * para poder dereferenciarlo, así mismo pasa con Box, viene siendo una referencia 

    */
}
