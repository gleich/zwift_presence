use std::{fs::File, thread::sleep, time::Duration};

use parse::Data;

mod discord;
mod parse;

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .init();
    let mut client = discord::connect().expect("Failed to connect to discord");
    loop {
        let fp = File::open("/Users/matt/Documents/Zwift Copy/Activities/inProgressActivity.fit")
            .expect("Failed to load file");
        let mut data = match Data::parse(fp) {
            Ok(d) => d,
            Err(_) => {
                println!("Failed to parse FIT file; might take a minute or two to generate valid records");
                sleep(Duration::from_secs(1));
                continue;
            }
        };
        data.convert_to_imperial();
        discord::update(data, &mut client).expect("Failed to update discord");
        log::info!("Updated");
        sleep(Duration::from_secs(5));
    }
}
