
#[derive(Debug)]
enum State{
    USA,
    UK
}
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(State) // 绑定值的模式匹配
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("match Penny.");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state quarter from {:?}", state);
            25
        },
    }
}

pub fn main() {
    let x = Coin::Quarter(State::UK);
    let cent = value_in_cents(x);
    println!("cent is {}", cent);

}