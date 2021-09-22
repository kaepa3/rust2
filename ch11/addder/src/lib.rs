#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}
impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}
pub fn add_two(a: i32) -> i32 {
    a + 2
}
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}
pub struct Guess{
    value : i32,
}
impl Guess {
    pub fn new(value:i32) -> Guess{
        if value < 1 || value >100{
            panic!("Guess value must be between 1 to 100, got {}", value);
        }
        Guess{
            value 
        }
    }
}
fn internal_addr(a:i32, b:i32) -> i32{ 
    a+b
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
    #[test] 
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
    #[test] 
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }
    #[test] 
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
    #[test]
    fn large_can_hold_smaller() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };
        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    #[ignore]
    fn error() {
        assert!(false);
    }
    #[test]
    fn internal() {
        assert_eq!!(4, internal_add(2,2));
    }
}
