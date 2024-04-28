use std::io;
use rand::{random, Rng};

fn main() {
    expression()
}

fn expression() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("{}", y);
}

fn tuple() {
    let tuple: (i32, u32, &str, f64) = (-10, 100, "아일릿", 1.2);

    println!("{}, {}, {}, {}", tuple.0, tuple.1, tuple.2, tuple.3);

    let (a, b, c, d) = tuple;

    println!("{}, {}, {}, {}", a, b, c, d);
}

fn array() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    a.iter().for_each(|&value| println!("{}", value));
}

fn baek_joon_11021() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("읽기 실패");

    let input: i32 = input.trim()
        .parse()
        .expect("읽기 실패");

    for a in (1..input + 1) {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("읽기 실패");

        let input = input.trim();

        let numbers: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().expect("숫자로 변환 실패"))
            .collect();

        let sum: i32 = numbers.iter().sum();

        println!("Case #{}: {}", a, sum);
    }
}

fn guess_number_game() {
    println!("숫자를 맞춰보세요!");

    let rand_int = rand::thread_rng()
        .gen_range(1..=10);

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("읽기 실패");

        let input: i32 = input.trim()
            .parse()
            .expect("읽기 실패");

        let mut result: bool = rand_int.eq(&input);

        if result {
            println!("\n맞추셨습니다!!");
        } else {
            println!("\n틀리셨습니다..");
        }

        println!("입력하신 숫자 : {}", input);
        println!("컴퓨터가 생각한 숫자 : {}", rand_int);

        if result {
            break;
        }
    }

}

fn mutable_variable() {
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