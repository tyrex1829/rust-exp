// function, Signed and unsigned number

fn main() {
    let ans: u32 = sum(2, 2);

    // Boolean
    let ans_bool: bool = is_even(20);

    // String
    let first_name = String::from("Tyrex");

    println!("ans: {}, ans_bool: {}, first_name: {}", ans, ans_bool, first_name)
}

fn sum(a: u32, b: u32) -> u32 {
    return a + b
}

// boolean

fn is_even (n: u32) -> bool {
    return n % 2 == 0;
}