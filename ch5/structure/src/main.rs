fn main() {
    rectangle();
    tuple_rectangle();
    struct_rectangle();
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
fn struct_rectangle() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("re:{:?}", rect.area());
}
fn tuple_rectangle() {
    let rect = (30, 50);
    println!("tuple:{}", area_rect(rect));
}
fn area_rect(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}
fn rectangle() {
    let width1 = 30;
    let height1 = 50;
    println!("{}", area(width1, height1));
}
fn area(width: u32, height: u32) -> u32 {
    width * height
}
