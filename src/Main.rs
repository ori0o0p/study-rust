fn main() {
    println!("{}", integer_sum(05, 11));
    println!("{}", string_sum("민", "주"));
}

fn integer_sum(a: i32, b: i32) -> i32 {
    return a + b
}

fn string_sum(a: &str, b: &str) -> String {
    return a.to_owned() + &*b
}