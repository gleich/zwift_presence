use std::{fs::File, u8};

use anyhow::{Context, Result};
use fitparser::{profile::MesgNum, FitDataRecord, Value};

#[derive(Default, Debug)]
pub struct Data {
    pub speed: f64,
    pub power: u16,
    pub distance: f64,
    pub cadence: f64,
    pub heart_rate: u8,
    pub altitude: f64,
}

impl Data {
    pub fn parse(mut file: File) -> Result<Self> {
        let records: Vec<FitDataRecord> = fitparser::from_reader(&mut file)
            .context("Failed to parse FIT file")?
            .iter()
            .cloned()
            .filter(|r| r.kind() == MesgNum::Record)
            .collect();
        let last_record = records.last().context("No records found")?;
        let mut data = Data::default();
        for field in last_record.fields() {
            match field.name() {
                "speed" => match field.value() {
                    Value::Float64(v) => data.speed = *v,
                    _ => println!("speed doesn't have value of f64"),
                },
                "power" => match field.value() {
                    Value::UInt16(v) => data.power = *v,
                    _ => println!("power doesn't have value of u16"),
                },
                "distance" => match field.value() {
                    Value::Float64(v) => data.distance = *v,
                    _ => println!("distance doesn't have value of u16"),
                },
                "cadence" => match field.value() {
                    Value::Float64(v) => data.cadence = *v,
                    _ => println!("cadence doesn't have value of u16"),
                },
                "heart_rate" => match field.value() {
                    Value::UInt8(v) => data.heart_rate = *v,
                    _ => println!("heart_rate doesn't have value of u8"),
                },
                "altitude" => match field.value() {
                    Value::Float64(v) => data.altitude = *v,
                    _ => println!("altitude doesn't have value of u16"),
                },
                _ => (),
            }
        }
        Ok(data)
    }
}
