use std::fmt;
use std::str::FromStr;
use regex::Regex;
use chrono::naive::time::{NaiveTime};

const LIGHTNING: u32 = 5 * 60;  // seconds

const LUNCH_TIME: u32 = 12 * 3600;  // seconds
const NETWORKING_TIME: u32 = 17 * 3600;  // seconds

const LUNCH_NAME: &'static str = "Lunch";
const NETWORKING_NAME: &'static str = "Networking";


lazy_static! {
    static ref RE: Regex = Regex::new(
        r"(?P<name>^.*) (?P<duration>[0-9]+|lightning)(?:min)?$"
    ).unwrap();
}


#[derive(Debug, Clone)]
pub struct Talk {
    pub name: String,
    pub duration: u32,  // seconds
    pub time: Option<NaiveTime>
}


impl Talk {
    pub fn new(name: &str, duration: u32, time: Option<NaiveTime>) -> Self {
        Talk{name: name.to_string(), duration: duration, time: time}
    }

    pub fn as_lunch() -> Self {
        Talk{
            name: LUNCH_NAME.to_string(),
            duration: 3600,
            time: Some(NaiveTime::from_num_seconds_from_midnight(LUNCH_TIME, 0))
        }
    }

    pub fn as_networking() -> Self {
        Talk{
            name: NETWORKING_NAME.to_string(),
            duration: 3600,
            time: Some(NaiveTime::from_num_seconds_from_midnight(NETWORKING_TIME, 0))
        }
    }
}


impl fmt::Display for Talk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(time) = self.time {
            write!(f, "{} ", time.format("%H:%M")).unwrap();
        }

        match self.duration {
            LIGHTNING => writeln!(f, "{} lightning", self.name),
            _ => writeln!(f, "{} {}min", self.name, self.duration / 60)
        }
    }
}


impl FromStr for Talk {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match RE.captures(s) {
            None => panic!(format!("Cannot parse {}", s)),
            Some(cap) => {
                let name = cap.name("name").unwrap().as_str();
                let duration = match cap.name("duration").unwrap().as_str() {
                    "lightning" => LIGHTNING,
                    s => s.parse::<u32>().unwrap() * 60
                };

                Ok(Talk::new(name, duration, None))
            }
        }
    }
}
