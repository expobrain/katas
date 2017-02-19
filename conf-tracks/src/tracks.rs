use std::fmt;

use talks::Talk;


pub const MORNING_START: u32 = 9 * 3600;  // seconds
pub const MORNING_END: u32 = 12 * 3600;  // seconds
pub const AFTERNOON_START: u32 = 13 * 3600;  // seconds
pub const AFTERNOON_END: u32 = 15 * 3600;  // seconds


#[derive(Debug)]
pub struct Track {
    pub morning: Vec<Talk>,
    pub afternoon: Vec<Talk>
}


impl Track {
    pub fn new() -> Self {
        Track{morning: vec![], afternoon: vec![]}
    }
}


impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn talks_fmt(f: &mut fmt::Formatter, talks: &Vec<Talk>) -> fmt::Result {
            for talk in talks {
                talk.fmt(f).unwrap();
            }

            Ok(())
        }

        talks_fmt(f, &self.morning).unwrap();
        talks_fmt(f, &self.afternoon).unwrap();

        Ok(())
    }
}
