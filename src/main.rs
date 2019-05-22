#![recursion_limit = "1024"]
extern crate csv;
extern crate failure;
#[macro_use]
extern crate serde;
extern crate chrono;

use failure::Error;
use std::fmt;
use std::fs::File;
use std::str::FromStr;
use std::process;
use chrono::{Local, Duration, DateTime, Utc};
use chrono::offset::TimeZone;

#[derive(Debug, Serialize, Deserialize)]
enum Type {
    Data,
    Definition,
}

impl FromStr for Type {
    type Err = Error;

    fn from_str(s: &str) -> Result<Type, Error> {
        match s {
            "Data" => Ok(Type::Data),
            "Definition" => Ok(Type::Definition),
            err => Err(failure::err_msg(format!(
                "'{}' makes no sense to be a type.",
                err
            ))),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum Message {
    FileID,
    DeveloperDataID,
    FieldDescription,
    Event,
    DeviceInfo,
    Unknown,
    Sport,
    Workout,
    Record,
    HRZone,
    PowerZone,
    Lap,
    Session,
    Activity,
}

impl FromStr for Message {
    type Err = Error;

    fn from_str(s: &str) -> Result<Message, Error> {
        match s {
            "file_id" => Ok(Message::FileID),
            "developer_data_id" => Ok(Message::DeveloperDataID),
            "field_description" => Ok(Message::FieldDescription),
            "event" => Ok(Message::Event),
            "device_info" => Ok(Message::DeviceInfo),
            "unknown" => Ok(Message::Unknown),
            "sport" => Ok(Message::Sport),
            "workout" => Ok(Message::Workout),
            "record" => Ok(Message::Record),
            "hr_zone" => Ok(Message::HRZone),
            "power_zone" => Ok(Message::PowerZone),
            "lap" => Ok(Message::Lap),
            "session" => Ok(Message::Session),
            "activity" => Ok(Message::Activity),
            err => Err(failure::err_msg(format!(
                "'{}' makes no sense to be a message.",
                err
            ))),
        }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

fn semicircles_to_degrees(semicircles: f32) -> f32 {
    let two: f32 = 2.0;
    semicircles * ( 180. / two.powi(31) )
}

fn load_data() -> Result<(), Error> {
    let csv_file = File::open("input/test.csv")?;
    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .from_reader(csv_file);

    //Seconds since UTC 00:00 Dec 31 1989 is the encoded timestamp
    let base_time: DateTime<Utc> = Utc.ymd(1989, 12, 31).and_hms(0,0,0);

    for result in rdr.records().take(100) {
        let record = result?;

        let type_= Type::from_str(&record[0])?;
        // [1] is 'local number'
        let message = Message::from_str(&record[2])?;

        if let Type::Data = type_ {
            if let Message::Record = message {
                // [3] is the 'timestamp' label
                let timecode = record[4].parse::<u32>()?;
                let timestamp = base_time + Duration::seconds(timecode as i64);
                //println!("{}", timestamp.to_string());
                //println!("{}", timestamp.with_timezone(&Local).to_string());
                //lat and lon are i32, but we only ever need them as floats - parse them directly.
                let pos_lat = record[7].parse::<f32>()?;
                let lat = semicircles_to_degrees(pos_lat);
                let pos_lon = record[10].parse::<f32>()?;
                let lon = semicircles_to_degrees(pos_lon);
                println!("{}: {}, {}", timestamp.with_timezone(&Local), lat, lon);
               // println!("{:?}", record);
            }
        }
    }
    Ok(())
}

fn main() {
    if let Err(err) = load_data() {
        println!("error loading data: {}", err);
        process::exit(1);
    }
}
