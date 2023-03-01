struct User {
    name: String,
    email: String,
    is_active: bool,
    user_role: UserRoles,
    website: Website
}

fn main() {
    
    // Enums
    
    /*
        la palabra reservada "enum", se utiliza para tipos enumerados. Esta es la diferencia con el 
        struct.
        
        - struct: se utiliza cuando un tipo de dato debe representar una cosa y otra cosa a la vez.
        - enum: se utiliza cuando un tipo de dato puede representar una cosa o alguna otra cosa diferente

        Las estructuras sirven para unir diferentes elementos en uno solo, mientras que los enumerados 
        permiten que un tipo de dato represente a diferentes cosas en diferente momento.

        Para declarar un enumerado se usa la palabra reservad enum seguido de un bloque "{}" con las 
        diferentes opciones separadas por coma. Como en el caso de los struct, la última opción puede 
        llevar la coma o no
    */

    let user_one = User {
        name: "John Doe".to_string(),
        email: String::from("johndoe@email.com"),
        is_active: true,
        user_role: UserRoles::BASIC,
        website: Website::INSTAGRAM(String::from("@johndoe")),
    };
    println!("{}, {}, {}", user_one.email, user_one.name, user_one.is_active);

    let access = has_acces(user_one.user_role);
}

enum UserRoles {
    BASIC,
    ADMIN,
}


// enum con agragación de datos
enum Website {
    URL(String),
    INSTAGRAM(String),
    LINKEDIN(String),
    FACEBOOK(String)
}

// enum como parámetro de una función
fn has_acces(user_role: UserRoles) -> bool {
    match user_role {
        UserRoles::ADMIN => true,
        UserRoles::BASIC => false,
    }
}

