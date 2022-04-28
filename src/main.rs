use std::{thread, time};

mod cache;

fn obnoxious_double(num: i32, c: &cache::Cache) {
    match c.retrieve(&num.to_string()) {
        Some(ans) => println!("I remember this one. {} * 2 = {}.", num, ans),
        None => {
            println!("I'm sorry. I'm bad at math.");
            thread::sleep(time::Duration::from_secs(1));
            println!("Just give me a bit to work on this...");
            thread::sleep(time::Duration::from_secs(1));
            println!("Almost there...");
            thread::sleep(time::Duration::from_secs(1));
            println!("Okay, I'm pretty sure that...");
            thread::sleep(time::Duration::from_secs(1));
            let val = num * 2;
            println!("{} * 2 = {}.", num, val);

            println!("I'm going to save that so I don't have to do this again.");
            match c.store(&num.to_string(), &val.to_string()) {
                Ok(_) => (),
                Err(_) => println!("Well, I tried, but I'm not sure I can remember this one."),
            }
        }
    }
}

fn main() {
    let cache = cache::Cache::new().unwrap();

    obnoxious_double(357, &cache);
    println!();
    obnoxious_double(357, &cache);
    println!();
    obnoxious_double(63425, &cache);
}
