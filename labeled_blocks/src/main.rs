fn main() {
    // Labeled blocks

    /* 
        esto es como crear diferentes scopes y también se les puede establecer bloques
    */

    let some_number: Option<i32> = Some(9);


    let result_of_blocks: i32 = 'process_a: {
        'first_loop: loop {
            let Some(num) = some_number else {
                break 'process_a 0;
            };

            if num > 100 {
                break 'first_loop 100;
            } else {
                break num;
            }
    
        }

        // a la variable num solo se puede acceder de este scope, ya fuera de el, no
    };

    println!("the result is: {}", result_of_blocks);
}
