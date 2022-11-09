use std::{fs::File, thread::sleep, time::Duration};

use parse::Data;

mod discord;
mod parse;

fn main() {
    let mut client = discord::connect().expect("Failed to connect to discord");
    loop {
        let fp = File::open("2022-11-08-16-52-39.fit").expect("Failed to load file");
        let data = Data::parse(fp).expect("Failed to parse FIT file");
        discord::update(data, &mut client).expect("Failed to update discord");
        println!("Updated");
        sleep(Duration::from_secs(5));
    }
}
