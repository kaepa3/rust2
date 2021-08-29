fn main() {
    push_str();
    moving();
    clone();
    stackonly();
    scope();
    return_scope();
    owner();
    owner2();
    enable_p();
    dangler();
    slice();
}
fn slice(){
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();
}
fn first_word(s: &String) -> usize{
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate(){
        if item ==b' '{
            return i;
        }
    }
    s.len()
}
fn dangler() {
    let ref_to_no = dangle();
    println!("{}", ref_to_no);
}
fn dangle() -> String {
    let s = String::from("hello");
    s
}
fn enable_p() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
fn owner2() {
    let s1 = String::from("hello");
    let len = calculate_length_p(&s1);
    println!("the length of {} is {}", s1, len);
}
fn calculate_length_p(some_string: &String) -> usize {
    let length = some_string.len();
    length
}
fn owner() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("the length of {} is {}", s2, len);
}
fn calculate_length(some_string: String) -> (String, usize) {
    let length = some_string.len();
    (some_string, length)
}
fn return_scope() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("{}:{}", s1, s3);
}
fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
fn scope() {
    let s = String::from("hello");
    takes_ownership(s);
    let x = 5;
    makes_copy(x);
}
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
fn stackonly() {
    let x = 5;
    let y = x;
    println!("x={}: y={}", x, y)
}

fn clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}, {}!", s1, s2);
}
fn moving() {
    let x = 5;
    let y = x;
    println!("{}", y);

    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s2);
}
fn push_str() {
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s);
}
