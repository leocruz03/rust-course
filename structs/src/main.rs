fn main() {
    
    // Structs

    /*
        con las estructuras se pueden crear nuevos tipos de datos. Se utilizan constantemente en Rust puesto 
        que son muy útiles. Las estructuras se crean con las palabra reservada "struct". El nombre de las es-
        tructuras debería estar en "UpperCammelCase".

        Existen tres tipos de estructuras:

        1. unit struct: no tiene nada, simplemente se escribe su nombre después de un punto y coma ";".
        
        2. tuple struct: o estructura sin nombre, solo es necesario escribir los tipos de datos que 
        contiene, sin nombres de campo. Esta son utilizadas o indicadas cuando se necesita una estructura 
        simple sin necesidad de utilizar nombres.

        3. normal struct: es la más habitual de todas. En estas estructuras se declaran los nombres de los 
        campos y sus tipos de datos en un bloque "{}". Estos bloques no se terminan con un punto y coma ";".

        Las estructuras también pueden ser mutables, aunque por defecto sean inmutables, con la palabra "mut" 
        la volvemos mutable, pero eso es solo cuando se utiliza en una variable al establecerle datos
    */

    // crear un color RGB
    let my_colour = Colour(50, 0, 50);
    println!("El green en este color equivale a: {}", my_colour.2);

    let united_state = Country {
        population: 300_000_000,
        capital: String::from("Washington D.C."),
        president: String::from("Joe Biden")
    };
    println!("El presidente de los estados unidos es: {}, su población es de: {} y su capital es: {}", 
        united_state.president, 
        united_state.population, 
        united_state.capital,
    );

    let mut user_one = User {
        name: "Joel".to_string(),
        email: "email@email.com".to_string(),
        year: 1984,
        is_active: true,
    };

    // cambiar el valor
    user_one.is_active = false;
    
    let user_two = new_user(String::from("Leonardo"), String::from("leonardo@leonardo.com"));
    println!("{}", user_two.email);

    // algo parecido al operador spread
    let user_three = User {
        // solo van a cambiar algunos datos, los demás siguen iguales
        name: "Juan".to_string(),
        email: "juan@juan.com".to_string(),
        ..user_one
    };
    println!("El nombre del usuario es: {} y nació en el año: {}", user_three.name, user_three.year);

    println!("La edad del usuario tres es: {}", user_three.calculate_age())
}

// estructura unitaria
struct FileDirectory;

// estructura tupla
struct Colour(u8, u8, u8);

// estructura habitual
struct Country {
    population: u128,
    capital: String,
    president: String,
}

struct User {
    name: String,
    email: String,
    year: u32,
    is_active: bool,
}

// init shorthand
fn new_user(name: String, email: String) -> User {
    return User {
        // como los parametros tienen el mismo nombre de los campos, entonces no hace falta poner el nombre de los campos
        name,
        email,
        year: 2000,
        is_active: true
    }
}

// funciones en structs
impl User {
    // por parametro se pasa la referencia del objeto mismo
    fn calculate_age(&self) -> u32 {
        let current_year = 2022;
        return current_year - self.year;
    }
}