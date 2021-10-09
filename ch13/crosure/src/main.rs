use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    workout(4, 5);
}
fn add_one_v1(x: u32) -> u32 {
    x + 1
}
fn workout(intensity: u32, random: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("slow calc");
        thread::sleep(Duration::from_secs(2));
        intensity
    });
    if intensity < 25 {
        println!("Today {}! push", expensive_result.value(intensity));
        println!("Next {}! push", expensive_result.value(intensity));
    } else {
        if random == 3 {
            println!("Rest!!!");
        }
        println!("Today {}! Run", expensive_result.value(intensity));
    }
}
