use std::ops::Deref;

fn main() {

    // Drop trait

    /* 
        nos indica que hacer cuando la instancia sale del scope
    */

    let x = 5;
    let y = MyBox::new(x);
    if *y == 5 {
        println!("Hola ")
    }    
}
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Ha salido del scope")
    }
}
