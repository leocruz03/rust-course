#![allow(unreahable_code, unused_variables, dead_code)]

/*
    los traits y enums por default sirven para que podamos dejar un valor indicado por defecto para 
    una estructura
*/


#[derive(Default, Debug)]
struct User {
    name: String,
    email: String,
    active: bool,
    user_role: UserRole,
    website: Website
}

#[derive(Default, Debug)]
enum UserRole {
    #[default]
    BASIC,
    ADMIN
}

/*
    si en el enum, hay un dato que se le especifica su tipo de dato, ese no se va a poder por 
    defecto
*/

#[derive(Default, Debug)]
enum Website {
    WEBSITE,
    #[default]
    FACEBOOK,
    INSTAGRAM,
    LINKEDIN
}

fn main() {
    let user = User {
        name: String::from("Leonardo"),
        email: String::from("email@email.com"),
        active: true,
        user_role: UserRole::ADMIN,
        website: Website::LINKEDIN,
    };

    // en vez de crearlo por la forma anterior se le puede dar los valores por defecto
    let user_two = User::default();
    println!("{:?}", user_two)
}
