fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("the value of x is: {}", x);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("space len:{}", spaces);

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess:{}", guess);

    let x = 2.0;
    let y: f32 = 3.0;
    print_type_of(&x);
    print_type_of(&y);

    calc();
    booler();
    charactor();
    multi();
    array();
    println!("{}", function(32));
    ifing();
    looper();
}
fn looper() {
    let mut count = 0;
    loop {
        println!("again");
        count += 1;
        if count > 3 {
            break;
        }
    }
    let a = [1, 2, 3, 4, 5];
    let mut index = 0;
    while index < 5 {
        println!("value:{}", a[index]);
        index = index + 1;
    }
    for element in a.iter() {
        println!("ele:{}", element);
    }
    for number in (1..4).rev() {
        println!("{}", number);
    }
}
fn ifing() {
    let condition = true;

    let number = if condition { 5 } else { 6 };
    println!("number is {}", number);
}
fn function(x: i32) -> i32 {
    println!("arg:{}", x);
    let y = {
        let x = 3;
        x + 1
    };
    println!("arg:{}", y);
    y
}
fn array() {
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("{}:{}", first, second);
}
fn multi() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{}:{}:{}", five_hundred, six_point_four, one);
}
fn charactor() {
    let c = 'z';
    let z = 'Z';
    let emoji = '🐱';
    println!("{}:{}:{}", c, z, emoji);
}
fn booler() {
    let t = true;
    let f: bool = false;
    println!("{}:{}", t, f)
}
fn calc() {
    let sum = 5 + 10;
    let difference = 95.4 - 4.4;
    let product = 24 * 44;
    let quotient = 24.78 / 88.5;
    let remainder = 43 % 5;

    println!(
        "{}:{}:{}:{}:{}:",
        sum, difference, product, quotient, remainder
    );
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
