fn main() {
    
    // Tests unitarios

    /*
        son un mecanismo de comprobación del funcionamiento de las unidades de menor tamaño de un programa o aplicación 
        en específico.

        Estas son las pruebas unitarias de software, forma parte de la estrategia de metodología ágil del trabajo del 
        desarrollo, donde se busca ofrecer pequeñas de software en funcionamiento en un corto periodo de tiempo, con 
        el objetivo de aumentar la satisfacción del cliente.
    */

    // macros para unit tests

    /* 
        - assert!(expression) --> asegura si la expresión es verdadera, si no lo es, lanza panic
        - assert_eq!(a, b) --> asegura que la expresión de la derecha es igual que a la de la izquierda
        - assert_ne!(a, b) --> asegura que la expresión de la derecha no es igual que la de la izquierda

        para asegurar de usar una función para test, solo se necesita poner arriba de la expresión "#[test]"
    */

    /*
        para ejecutar los tests, se utiliza el comando "cargo test" y se correran todos los tests

        para ejecutar un test en concreto se utiliza el comando "cargo test 'name_test'" (sin las comillas) 
        y correra el test mencionado

        para ejecutar tests con palabras en común se utiliza el comando "cargo test 'comun_words'" (sin las comillas) 
        y correran los tests con las palabras en común ya mencionadas
    */

}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn good_add() {
    // assert_eq!(add(3, 3), 9) // ❌
    assert_eq!(add(3, 3), 6); // ✅
}


fn just_numbers(code: &str) -> bool {
    code.chars().all(char::is_numeric)
}

#[test]
fn check_just_number() {
    let result = just_numbers("98302334");
    assert!(result);
}

#[test]
fn check_with_letters() {
    let code = "983s0L2334";
    // con assert también se pueden modificar los mensajes de error
    let result = just_numbers(code);
    assert!(result, "el código no fue valido porque contiene letras, el código ingresado fue: {code}");
}
