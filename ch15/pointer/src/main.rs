use std::{cell::RefCell, ops::Deref};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}
use std::rc::Rc;
use List::{Cons, Nil};

struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping data {}!", self.data);
    }
}
pub trait Messenger {
    fn send(&self, msg: &str);
}
pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}
impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let persentage_of_max = self.value as f64 / self.max as f64;

        if persentage_of_max >= 0.75 && persentage_of_max < 0.9 {
            self.messenger.send("warning");
        } else if persentage_of_max >= 0.9 && persentage_of_max < 1.0 {
            self.messenger.send("over 90!");
        } else if persentage_of_max >= 1.0 {
            self.messenger.send("quota!");
        }
    }
}
#[derive(Debug)]
enum Hoge {
    Cons(Rc<RefCell<i32>>, Rc<Hoge>),
    Nil,
}


fn main() {
    {
        let value = Rc::new(RefCell::new(5));
        let a = Rc::new(Hoge::Cons(Rc::clone(&value), Rc::new(Hoge::Nil)));
        
        let b = Hoge::Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
        let c = Hoge::Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        println!("a after ={:?}", a);
        println!("b after ={:?}", b);
        println!("c after ={:?}", c);
    }
    {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("count {}", Rc::strong_count(&a));
        let b = Cons(3, Rc::clone(&a));
        println!("count after b{}", Rc::strong_count(&a));
        {
            let c = Cons(4, Rc::clone(&a));
            println!("count after c{}", Rc::strong_count(&a));
        }
        println!("count after func{}", Rc::strong_count(&a));
    }
    {
        let c = CustomSmartPointer {
            data: String::from("some data"),
        };
        println!("created!!");
        drop(c);
        println!("out func")
    }
    {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("created!!")
    }
    myb();
    bpo();
    pointer();
    boxer();
}
fn myb() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
fn hello(name: &str) {
    println!("Hello {}", name);
}
fn bpo() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
fn pointer() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
fn boxer() {
    let b = Box::new(5);
    println!("b={}", b);
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }
    #[test]
    fn it_sends_an_over_75_persent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
