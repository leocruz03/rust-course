use std::rc::Rc;

fn main() {

    // Referenced counted Smart pointer

    /*
        permite que un valor pueda tener diferentes dueños, se usa cuando queremos asignar datos al heap 
        para que sea accedido en multiples partes del código, y no podemos determinar en tiempo de 
        compilación el último que accedera a estos datos. Si supieramos de ante mano quien sería el último, 
        podríamos hacer que ese último sea el dueño, pero no lo sabemos. Netonces referenced counted lleva 
        un contador de referencias con todos los dueños, y cuando ya no quedan más dueños, puede limpiar 
        el dato
    */

    #[derive(Debug)]
    enum List {
        Node(i32, Rc<List>),
        None,
    }

    use List::*;

    // node_one  -->
    //               node_two --> node_three --> None
    // node_zero -->
    
    let node_three = Node(10, Rc::new(None));
    /*
        para hacer el ejemplo que se muestra en el diagrama, necesitamos clonar el node_two, pero no de la forma 
        común, si no con Rc y se necesita pasarle la referencia
    */
    let node_two = Node(3, Rc::new(node_three));
    let node_two_rc = Rc::new(node_two);

    // mostrar la cantidad de referencias
    println!("número de referencias de node_two_rc: {}", Rc::strong_count(&node_two_rc));
    
    let node_one = Node(90, Rc::clone(&node_two_rc));
    let node_zero = Node(6, Rc::clone(&node_two_rc));
    println!("{:?}", node_zero);
    println!("{:?}", node_one);
    println!("número de referencias de node_two_rc: {}", Rc::strong_count(&node_two_rc));
}
