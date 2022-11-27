use std::{fs::File, u8};

use anyhow::{Context, Result};
use fitparser::{profile::MesgNum, FitDataRecord, Value};

#[derive(Default, Debug)]
pub struct Data {
    pub speed: Option<f64>,
    pub power: Option<u16>,
    pub distance: Option<f64>,
    pub cadence: Option<u8>,
    pub heart_rate: Option<u8>,
    pub altitude: Option<f64>,
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
                "enhanced_speed" => match field.value() {
                    Value::Float64(v) => data.speed = Some(*v),
                    _ => println!("speed doesn't have value of f64"),
                },
                "power" => match field.value() {
                    Value::UInt16(v) => data.power = Some(*v),
                    _ => println!("power doesn't have value of u16"),
                },
                "distance" => match field.value() {
                    Value::Float64(v) => data.distance = Some(*v),
                    _ => println!("distance doesn't have value of u16"),
                },
                "cadence" => match field.value() {
                    Value::UInt8(v) => data.cadence = Some(*v),
                    _ => println!("cadence doesn't have value of u8"),
                },
                "heart_rate" => match field.value() {
                    Value::UInt8(v) => data.heart_rate = Some(*v),
                    _ => println!("heart_rate doesn't have value of u8"),
                },
                "altitude" => match field.value() {
                    Value::Float64(v) => data.altitude = Some(*v),
                    _ => println!("altitude doesn't have value of u16"),
                },
                _ => (),
            }
        }
        Ok(data)
    }

    pub fn convert_to_imperial(&mut self) {
        if self.speed.is_some() {
            // meters per second to miles per hour
            self.speed = Some(self.speed.unwrap() * 2.2369)
        }
        if self.distance.is_some() {
            // meters to miles
            self.distance = Some(self.distance.unwrap() / 1609.0)
        }
        if self.altitude.is_some() {
            // meters to feet
            self.altitude = Some(self.altitude.unwrap() * 3.281)
        }
    }
}
