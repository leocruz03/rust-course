fn main() {
    
    // Flujos de control

    /*
        los flujos de control son los que nos permiten tener más control sobre los procesos que se realiza 
        en nuestro código, a veces necesitamos repetir cierta acción, una cierta cantidad de veces; a veces 
        también vamos a necesitar manejar permisos y para eso podemos utilizar los flujos de control
    */

    // condicionales if ...else if ...else
    let age = 40;

    if age < 18 {
        println!("your age not enough for enter")
    } else if age >= 18 && age <= 25 {
        println!("'u can enter")
    } else {
        println!("was a older for enter")
    }

}
