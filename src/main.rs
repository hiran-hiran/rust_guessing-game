use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("数字を予想してください");
    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("The secret number is: {}", secret_number);
    loop {
        println!("あなたの予想を入力してください");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("入力を読み取れませんでした");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("数字を入力してください");
                continue;
            }
        };

        println!("あなたの予想は　{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さいです"),
            Ordering::Greater => println!("大きいです"),
            Ordering::Equal => {
                println!("正解！");
                break;
            }
        }
    }
}
