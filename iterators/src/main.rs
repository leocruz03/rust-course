use std::collections::btree_map::Iter;

fn main() {

    // Iterators (iteradores)

    /*
        cuando se habla de iteradores, hablamos sobre la secuencia lógica de iterar sobre una 
        secuencia de elementos hasta que se acaben y hacer alguna acción en cada uno de ellos
    */

    let arr = [1, 2, 3, 4, 5];

    // para iterar sobre algo se utiliza el método ".iter()"
    for i in arr.iter() {
        println!("{}", i)
    }

    println!("--------------------------------------------------------------------------------------------------");

    // sumandole uno
    for i in arr.iter() {
        println!("{}", i + 1)
    }

    println!("---------------------------------------------------------------------------------------------------");

    let mut c = Counter::new();
    let i = c.next();
    // como retorna some se hace un match

    match i {
        Some(count) => println!("{:?}", count),
        _ => println!("nothing"),
    }

}

struct Counter {
    count: i32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        Some(self.count)
    }
}