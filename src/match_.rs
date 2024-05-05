enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

pub fn main() {
    let x = Coin::Quarter;
    let cent = value_in_cents(x);
    println!("cent is {}", cent);

}