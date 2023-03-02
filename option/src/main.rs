fn main() {
    
    // Option

    /*
        el option tiene dos valores posibles, Some y None, y se utiliza cuando se da el caso de que un 
        valor pueda existir o no, cuando un valor existe se usa Some(valor). Cuando no existe None
    */

    let mut name: Option<String> = None;

    name = Some("leonardo".to_string());

    match name {
        None => println!("Nombre no indicado"),
        Some(name) => println!("{}", name)
    }

    // ----------------------------------------------------------------------------------------------------------------

    /* let user_one = User {
        name: String::from("John"),
        age: 18,
    };

    println!("{}", user_one.name);

    let age_user_one = user_one.get_age();
    println!("Edad de user_one: {}", age_user_one); */

    /* 
        pero digamos que el usuario no envió la edad, esto es una buena forma para implementar el option
    */

    let user_one = User {
        name: String::from("leonardo"),
        age: Some(18)
    };

    let age_user_one = user_one.get_age();

    match age_user_one {
        Some(age_user_one) => println!("La edad de user_one es: {}", age_user_one),
        _ => println!("La edad no fue especificada")
    }

}

// options en structs
struct User {
    name: String,
    age: Option<u32>
}

impl User {
    fn get_age(&self) -> Option<u32> {
        return self.age
    } 
}