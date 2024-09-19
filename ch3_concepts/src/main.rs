const SOME_STUFF: u128 = 100000;

fn main() {
    println!("Hello, world!");

    another_function();

    func_with_param(32);
    println!("Random char: {}", func_with_return());

    expression_if();

    loop_a_while();
    loop_with_return();
    loops_with_labels();
    for_loop_with_range();
}

fn another_function() {
    let var_int: u64 = 2134;
    let var_float: f32 = 34.12431;
    let var_bool: bool = true;
    let var_char: char = '😻';

    let var_tuple: (i32, f64, char, bool) = (2332, 9.232, 'h', false);
    let var_tuple_char: char = var_tuple.2;
    let var_tuple_unit: () = ();

    let var_arr: [char; 3] = ['a', 'b', 'c'];
    let var_arr_b: char = var_arr[1];
    let var_all_3s: [i32; 5] = [3; 5];  // [3, 3, 3, 3, 3]

    println!("Another function. Here's a value: {}", SOME_STUFF);
}

fn func_with_param(x: i32) {
    println!("The given value is: {x}");
}

fn func_with_return() -> char {
    'b'
}

fn expression_if() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn loop_a_while() {
    let mut num: i32 = 0;
    loop {
        println!("looped!");
        num += 1;

        if num >= 5 {
            break
        }
    }
}

fn loop_with_return() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn loops_with_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn for_loop_with_range() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
