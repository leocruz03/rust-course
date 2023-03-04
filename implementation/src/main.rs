fn main() {
    /*
        la palabra clave "impl" en rust se utiliza para implementar algunas funciones en los tipos. 
        Esta funcionalidad puede incluir tanto funciones como costos. Hay dos tipos principales de 
        implementaciones en rust:

        - implementaciones inherentes
        - implementaciones de rasgos

        En esta toma, nos centraremos en implementaciones inherentes.

        Implementaciones Inherentes
        
        como su nombre lo indica, son independientes. Están atados a un solo hormigón "self" tipo que se 
        especifica después del "impl". Estas implementaciones, a diferencias de las funciones estándar, 
        siempre están dentro del alcance.
    */

    // ejemplo

    struct Person {
        name: String,
        age: u32,
    }

    // implemetación de funcionalidad en la struct person con la palabra clave "impl"
    impl Person {
        fn introduction(&self) {
            println!("Hi, my name is {} and i'm {} years old", self.name, self.age);
        }
    }

    let person_one = Person {
        name: String::from("Leonardo"),
        age: 19
    };

    person_one.introduction();
}
