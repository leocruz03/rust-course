fn main() {
    // Genericos

    /*
        es un poco difícil explicar este concepto, pero un ejemplo puede ser que tengamos un 
        plano cartesiano y necesitamos dos puntos, uno de esos puntos son números flotantes 
        y otros son números enteros, pero los dos puntos queremos sacarlos del mismo struct
    */

    let coordinate_a = Point {
        x: 8.90,
        // y: 8, el error es que espera el mismo tipo da dato que el punto x
        y: 7.234,
    };

    println!("Las coordenadas del punto A es, en X: {:?} y en Y: {}", coordinate_a.x, coordinate_a.y);

    let coordinate_b = Point {
        x: 3,
        y: 10,
    };

    let coordinate_d = Point {
        x: 7,
        y: 340,
    };

    println!("Las coordenadas del punto B es, en X: {:?} y en Y: {}", coordinate_b.x, coordinate_b.y);

    // en este caso no da error la diferencia de tipos, ya que se le establecieron dos genericos
    let coordinate_c = FlexiblePoint {
        x: 6,
        y: 5.097,
    };

    println!("Las coordenadas del punto C es, en X: {} y en Y: {}", coordinate_c.x, coordinate_c.y);

    calculate_area(coordinate_d, coordinate_a);
}

/*
    aquí se asume que la estructura point puede tener cualquier tipo de dato, pero cuando hagamos referencia 
    a la estructura la variable tiene que tener los dos datos del mismo tipo porque lo tenemos todo de un solo, 
    pero también se le puede poner más de un tipo de dato generico
*/
struct Point<T> {
    x: T,
    y: T,
}

struct FlexiblePoint<T, V> {
    x: T,
    y: V,
}

// genericos en métodos
fn calculate_area<T, V>(point_a: Point<T>, point_b: Point<V>) {
    // todo
}