use std::cmp::min;

fn main() {
    let mut min_ju = 1;
    min_ju += 100;

    println!("{}", integer_sum(min_ju, 11));

    let min_ju = "민주";
    println!("{}", string_sum("박", min_ju));
}

fn scope() {
    let a = 1;
    {
        let a = "아일릿";
        println!("{}", a);
    }

    println!("{}", a);
}

fn integer_sum(a: i32, b: i32) -> i32 {
    return a + b
}

fn string_sum(a: &str, b: &str) -> String {
    return a.to_owned() + &*b
}