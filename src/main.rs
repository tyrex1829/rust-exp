// function, Signed and unsigned number

// fn main() {
//     let ans: u32 = sum(2, 2);

//     // Boolean
//     let ans_bool: bool = is_even(20);

//     // String
//     let first_name = String::from("Tyrex");

//     println!("ans: {}, ans_bool: {}, first_name: {}", ans, ans_bool, first_name)
// }

// fn sum(a: u32, b: u32) -> u32 {
//     return a + b
// }

// // boolean

// fn is_even (n: u32) -> bool {
//     return n % 2 == 0;
// }


fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    another_fn(5, 'T');

    let y = five();
    println!("The vale of x is: {y}");

    let z = plus_one(5);
    println!("The vale of z is: {z}");

}

fn another_fn (x: i32, c: char) {
    println!("Another fn: {x}{c}")
}

fn five () -> i32 {
    5
}

fn plus_one (x: i32) -> i32 {
    x + 1
}