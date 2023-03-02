use std::fmt;

fn main() {
    // Macro

    /*
        una macro en rust es una función que escribe código rust, es como un console.log en JS, un 
        macro se puede identificar porque tiene un signo de exclamación "!" 
    */

    /*
        uno de los macros más utilizados es "println!", el cual formatea los elementos que le pasamos 
        por parámetros el cual viene derivado de format o "fmt" el cual también tiene varios derivados 
        de el.
        
        fmt: se utiliza para formatear strings con argumentos

        Trait Debug, para este se utiliza {":?"} este signo de interrogación y los dos puntos van en todos 
        los trait de este tipo. 
    */

    println!("Hola {}", "leonardo");

    // trait debug
    println!("Hello {:?}", "John");

    let user = User {
        name: String::from("Leonardo José"),
        age: 19,
    };
    
    /*
    el trait debug nos sirve para imprimir cosas como estas, ya que solo con las llaves no lo podemos 
    hacer 
    */
    
    println!("{:?}", user);
    
    // atributo derive
    
    /* 
    permite generar automaticamente nuevos elementos para las estructuras de datos. Utiliza la sintaxis 
    MetaListPaths para especificar una lista de rasgos para implementar o rutas para derivar macros 
    para procesar.
    
    - los traits de comparación , Eq, PartialEq, Ord, PartialOrd.
    - Clone
    - Copy
    - Hash
    - Default
    - Debug
    */
    
}

/*  
    el trait display

    es más diferente al debug, porque es más usado cuando uno quiere manejar más lo que se va a mostrar
*/

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
      writeln!(f, "{} ({})", self.name, self.age)
    }
  }

#[derive(Debug)]
struct User {
    name: String,
    age: i32,
}
