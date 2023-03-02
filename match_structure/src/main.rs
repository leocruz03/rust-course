#[allow(dead_code)] // quitar los warnings de código no usado

fn main() {

    // Match - estructura de control

    /*
        match es una estructura de control en rust, la cual es más límpia que usar if y else, match requiere 
        que se contemplen los errores, es muy bueno para evaluar y puede retornar o no un valor.
    */

    let my_number: u8 = 5;

    /*
        aquí se está evaluando el valor de la variable, si poseé ese valor, y si en ninguno de los casos se iguala 
        se va tomar un caso por defecto el cual se establece, por si no llegase a coincidir ninguno
    */
    match my_number {
        0 => println!("Es cero"),
        1 => println!("Es uno"),
        2 => println!("Es dos"),
        _ => println!("Es algún otro número"),
    }

    let first_colour = (200, 0, 0);
    let second_colour = (50, 50, 50);
    match_colours(first_colour);
    match_colours(second_colour);
}

// esta función recibe una tupla como parámetro
fn match_colours(rgb: (i32, i32, i32)) {
    match rgb {
        (r, _, _) if r < 10 => println!("No muy rojo"),
        (_, g, _) if g < 10 => println!("No muy verde"),
        (_, _, b) if b < 10 => println!("No muy azul"),
        _ => println!("Cada color tiene al menos 10")
    }
}