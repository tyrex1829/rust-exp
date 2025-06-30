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

// functions 

// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");

//     x = 6;
//     println!("The value of x is: {x}");

//     another_fn(5, 'T');

//     let y = five();
//     println!("The vale of x is: {y}");

//     let z = plus_one(5);
//     println!("The vale of z is: {z}");

// }

// fn another_fn (x: i32, c: char) {
//     println!("Another fn: {x}{c}")
// }

// fn five () -> i32 {
//     5
// }

// fn plus_one (x: i32) -> i32 {
//     x + 1
// }

// conditions

// fn main () {
//     let number = 7;

//     if number < 5 {
//         println!("true")
//     } else {
//         println!("false")
//     }

//     let condition = true;
//     let check = if condition {5} else {6};
//     println!("value is: {check}")
// }

// Loops -> loop, while, for

fn main () {

    // loop
    // loop {
    //     println!("again");
    //     break;
    // }

    // let mut counter = 0;
    // let res = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    // println!("The res is {res}");

    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }
    //     count += 1;
    // }
    // println!("End count = {count}");

    // while
//     let mut number = 3;
//     while number != 0 {
//         println!("{number}!");

//         number -= 1;
//     }
//     println!("LOFTOF!!");

//     let a = [10, 20, 30];
//     let mut i = 0;
//     while i < a.len() {
//         println!("value: {}", a[i]);
//         i += 1;
//     }

// for
let  a = [10, 20, 30];
for e in a {
    println!("value: {e}")
}
for n in (1..4).rev() {
    println!("{n}")
}
println!("hey")
}