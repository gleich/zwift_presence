use std::fs::File;

use parse::Data;

mod parse;

fn main() {
    let fp = File::open("2022-11-08-16-52-39.fit").expect("Failed to load file");
    let data = Data::parse(fp).expect("Failed to parse file");
    dbg!(data);
}
