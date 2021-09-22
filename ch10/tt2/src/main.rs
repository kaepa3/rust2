use std::fmt::Display;
struct ImporantExcept<'a>{
    part: &`a str,
}

struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x={}", self.x);
        } else {
            println!("The largest member is y={}", self.y)
        }
    }
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn main() {
    let novel =String::from("call. hoge ");
    let first_sentence = novel.split('.')
        .next().expect("not find");
    let i = ImporantExcept{ part:first_sentence };

    let str1 = String::from("abcd");
    let str2 = "xyz";
    let result = longest(str1.as_str(), str2);
    println!("long is {}", result);
}
