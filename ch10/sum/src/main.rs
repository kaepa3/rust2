enum Option<T> {
    Some(T),
    None,
}
struct Point<T, U> {
    x: T,
    y: U,
}
impl<T, U> Point<T, U> {
    fn hoge(&self) -> &T {
        &self.x
    }
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<V, W> {
        Point {
            x: other.x,
            y: other.y,
        }
    }
}
impl Point<f32, f32> {
    fn distance(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 5.1, y: 1.3 };
    let i_f = Point { x: 5.1, y: 1.2 };

    let p = Point { x: 5, y: 10 };
    println!("={}", p.hoge());

    let numbers = vec![34, 50, 25, 100, 65];
    println!("{}", largest(&numbers));

    let numbers = vec![102, 34, 50, 6000, 54, 2, 65];
    println!("{}", largest(&numbers));

    let chars = vec!['y', 'z', 'k', 'c'];
    println!("{}", largest(&chars));
}
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn largest_i32(numbers: &[i32]) -> i32 {
    let mut largest = numbers[0];
    for &number in numbers.iter() {
        if number > largest {
            largest = number;
        }
    }
    largest
}
fn largest_char(numbers: &[char]) -> char {
    let mut largest = numbers[0];
    for &number in numbers.iter() {
        if number > largest {
            largest = number;
        }
    }
    largest
}
