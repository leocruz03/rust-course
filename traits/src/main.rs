fn main() {
    
    // Traits

    /*
        Son una serie de métodos que tenemos para un tipo de dato desconocido, es algo muy parecido 
        a las interfaces en otros lenguajes poer la diferencia es que se pueden ampliar traits a otros 
        ya existentes o podemos crear los nuestros
    */

    let circle_one = Circle {
        x: 0.0,
        y: 0.0,
        radio: 2.5,
    };

    println!("El área del circulo es: {}, pero sus cordenadas son: {}, {}", circle_one.area(), circle_one.x, circle_one.y);

    let square_one = Square {
        side: 6.98,
    };

    println!("El lado del cuadrado es: {} y su área es de: {}", square_one.side, square_one.area());

    let human_one = Human {
        name: String::from("Leonardo"),
        age: 65,
        id: String::from("123456789")
    };

    println!("El humano de nombre: {}, con id: {} y de edad: {}; su año de nacimiento es: {}", 
        human_one.name, 
        human_one.id, 
        human_one.age, 
        human_one.calculate_born_year()
    );

    show_area(circle_one);
    show_area(square_one);
}

struct Circle {
    x: f64,
    y: f64,
    radio: f64,
}

trait Area {
    fn area(&self) -> f64;
}

// implementar el trait a la estructura
impl Area for Circle {
    fn area(&self) -> f64 {
        return (self.radio * self.radio) * 3.14159 
    }
}

struct Square {
    side: f64,
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// función con circulo y cuadrado
fn show_area<T: Area>(figure: T) {
    println!("Esta figura tiene área de: {}", figure.area());
}

// ejemplo
struct Human {
    name: String,
    age: u32,
    id: String,
}

trait CalculateBornYear {
    fn calculate_born_year(&self) -> u32;
}

impl CalculateBornYear for Human {
    fn calculate_born_year(&self) -> u32 {
        2023 - self.age
    }
}