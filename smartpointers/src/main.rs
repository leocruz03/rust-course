fn main() {

    // SmartPointers

    /*
        los punteros inteligentes en español, es un tipo de puntero, en el sentido más tradicional 
        del concepto puntero, un valor en el stack que apunta a una dirección de memoria en el 
        heap, pero con unas capacidades adicionales. La más importante de todas es que encapsulan una 
        referencia y se hacen los "owners" de la misma. De esta manera, la referencia puede ser usada 
        en diferentes lugares del código y es liberada una vez su dueño desaparece, se podría decir 
        que un smart pointer es un puntero seguro, los smartpointers son usualmente implementados en 
        structs pero implementando los traits Deref y Drop. 

        Deref, permite a las instancias de smart pointer comportarse como referencias, para que el 
        mismo código que funciona con referencias, funcione con smart pointers.

        Drop trait, permite definir lógica que se ejecute una vez que el smart pointer sale del scope
    */

    // Box<T>
    let x = 5; // va a estar almacenado en el stack porque es un valor fijo
    let y = Box::new(3); // aquí ya se está guardando en el heap
    println!("x = {}, y = {}", x, y);

    /*
        sirve para definir cuanta memoria necesita en tiempo de compilación
    */

    // también se pueden utilizar en listas enlazadas
    enum List {
        Node(i32, Box<List>),
        None,
    }

    let node3 =  List::Node(10, Box::new(List::None));
    let node2 = List::Node(3, Box::new(node3));
    let node1 = List::Node(90, Box::new(node2));


}
