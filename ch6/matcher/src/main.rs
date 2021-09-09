enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater(u32),
}
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("lucky!!");
            100
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quater(state) => {
            println!("state is {}!!", state);
            state
        },
    }
}
fn main() {
    let val = value_in_cents(Coin::Penny);
    println!("{}",val);
    let val = value_in_cents(Coin::Quater(200));
    println!("{}",val);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x{
        None =>None,
        Some(i) => Some(i+1),
    }
}
