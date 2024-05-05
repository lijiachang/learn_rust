mod variables;
mod function;
mod if_else;
mod loop_while_for;

use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("guess number");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("the secret number is {secret_number}");
    loop {
        println!("input your guess");
        // let mut foo = 1;  // 可变对象
        // foo = 2;
        // let bar = foo; // 把foo变量的值赋值给一个不可变变量bar

        let mut guess = String::new();
        // 方法的参数按照引用进行传递
        io::stdin().read_line(&mut guess).expect("failed to read");

        println!("your guess:{guess}");

        // let guess_number = i32::from_str_radix(&guess.trim(), 10).expect("cant convert to a number!");
        // if secret_number == guess_number {
        //     println!("win!");
        // } else if guess_number < secret_number  {
        //     println!("too small")
        // } else if guess_number > secret_number {
        //     println!("too big")
        // }

        // 把string转换为int；变量的重复使用；parse()可以根据前面声明的类型自动推导？
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("win");
                break;
            }
        }
    }
}