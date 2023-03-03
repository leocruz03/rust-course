use::std::collections::HashSet;

fn main() {
    
    // HashSet

    /* 
        son como un hashmap pero donde solo importan las keys, es para garantizar que no hayan elementos 
        duplicados 
    */

    // inicializar el hashset
    let mut user_ids: HashSet<i32> = HashSet::new();

    // insertar datos en el hashset, necesita ser mutable
    user_ids.insert(100);
    user_ids.insert(93);
    user_ids.insert(23);

    // remover datos de hashset, se necesita pasar por referencia
    user_ids.remove(&93);

    let mut backup_users = HashSet::new();
    backup_users.insert(100);
    backup_users.insert(23);
    backup_users.insert(9);

    // TODOS ESTOS MÉTODOS RETORNAN UN ITER Y SE LES NECESITA PASAR LA REFERENCIA

    // obtener los elementos únicos entre 2 hashsets, se usa el método ".union()"
    for id in user_ids.union(&backup_users) {
        println!("once elements in user_ids and backup_users: {}", id);
    }

    // obtener los elementos que están en el primer set, y no en el otro, se usa el método ".difference()"
    for id in user_ids.difference(&backup_users) {
        println!("just elements into first set: {}", id);
    }

    // obtener solo los elementos que están en ambos sets, se usa el método ".intersection()"
    for id in user_ids.intersection(&backup_users) {
        println!("elements thst 2 sets share: {}", id);
    }

    /*
        obtener todos los elementos que están en un set, o en el otro, pero no en ambos, se usa el 
        método ".symetric_difference()"
    */
    for id in backup_users.symmetric_difference(&user_ids) {
        println!("hhhh: {}", id);
    }

    // recorrer un hashset, nos pide el método ".iter()"
    for id in user_ids.iter() {
        println!("{}", id)
    }
}
