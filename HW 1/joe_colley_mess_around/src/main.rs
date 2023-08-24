use std::result;

fn main() {
    let input: [i32; 5] = [1, 3, 2, 5, 4];

    hello_world();
    array(input);

    loops_array(input);

    let vec = sort(input);
    loops_vec(vec.clone());
    loops_loop(vec.clone());

    let n = 6;
    let output = factorial(n);
    println!("Factorial of {} is {}", n, output);
}

fn hello_world() {
    let hello_world = String::from("Hello World");
    println!("{}", hello_world);
}

fn array(array: [i32; 5]) {
    println!("Array[2] = {}", array[2]);
    println!("Array[3] = {}", array[3]);

    let length = array.len();
    println!("Length: {}", length);
}

fn loops_loop(vec: Vec<i32>) {
    println!("Loop: ");
    let mut i = 0;

    loop {
        println!("i: {}", vec[i]);
        i += 1;

        if i > vec.len() - 1 {
            break;
        }
    }
}

fn loops_array(array: [i32; 5]) {
    println!("Array Elements: ");
    for element in &array {
        println!("{}", element);
    }
}

fn loops_vec(vec: Vec<i32>) {
    println!("Vec Elements: ");
    for element in &vec {
        println!("{}", element);
    }
}

fn sort(array: [i32; 5]) -> Vec<i32> {
    let mut vec = array.to_vec();
    vec.sort();
    vec
}

fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}
