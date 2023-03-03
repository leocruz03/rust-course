fn main() {

    // Strings
    
    /*
        rust maneja los strings de dos formas, "String" y string slice (slice hace referencia a una parte, y 
        en collections es una referencia a una contigua secuencia de elementos). "String" se guarda en el heap 
        por lo cual, le permite crecer y achicarse, pero el "string slide" se guarda en el stack ya que es una 
        referencia, los cualse son inmutables
    */

    // formas de crear un String
    let name = String::from("John");
    let last_name = "Doe".to_string();

    println!("{} {}", name, last_name);

    // formas de crear un string slide
    let id = "6672469012";
    println!("john id is: {}", id);

    // pasar de string slide a String
    let mut other_id = id.to_string();
    println!("{}", other_id);

    // pasar de String a string slice
    let mod_lastname = &last_name[..last_name.len()]; // se pasa la referencia del String
    println!("{}", mod_lastname);

    // añadir elementos a String
    other_id.push('3');
    println!("{}", other_id)
}
