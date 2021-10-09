use std::thread;
use std::time::Duration;
fn main() {
    workout(4,5);
}
fn workout(intensity: u32, random: u32) {
    let result = simu(intensity);
    if intensity < 25 {
        println!("Today {}! push", result);
        println!("Next {}! push",  result);
    } else {
        if random == 3 {
            println!("Rest!!!");
        }
        println!("Today {}! Run",  result);
    }
}

fn simu(intensity: u32) -> u32 {
    println!("slow calc");
    thread::sleep(Duration::from_secs(2));
    intensity
}
