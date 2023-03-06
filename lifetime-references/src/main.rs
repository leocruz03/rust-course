fn main() {
    
    // Lifetime de las referencias

    /*
        es el tiempo de vida de las referencias a memoria
    */
}

fn do_some<'a>(param: &'a i32) -> &'a i32 {
    param
}
