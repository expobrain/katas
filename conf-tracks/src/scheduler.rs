use chrono::naive::time::{NaiveTime};

use tracks::*;
use talks::{Talk};


#[derive(Debug)]
pub struct Scheduler {
    talks: Vec<Talk>
}


impl Scheduler {
    pub fn from_talks(mut talks: Vec<Talk>) -> Self {
        // Sort the talks by time descending to help the scheduler later
        talks.sort_by(|a, b| b.duration.cmp(&a.duration));

        Scheduler{talks: talks}
    }

    pub fn schedule(&mut self) -> Vec<Track> {
        fn inner_scheduler(talks: &mut Vec<Talk>, start: u32, end: u32) -> Vec<Talk> {
            // Initialise state
            let mut selected = Vec::with_capacity(talks.len());
            let mut index = 0;
            let mut talk_time = start;
            let mut remaining_time = end - start;

            // Select talks
            while index < talks.len() && remaining_time > 0 {
                let duration = talks[index].duration;

                if duration <= remaining_time {
                    let mut talk = talks.remove(index);
                    talk.time = Some(NaiveTime::from_num_seconds_from_midnight(talk_time, 0));

                    selected.push(talk);

                    remaining_time -= duration;
                    talk_time += duration;
                } else {
                    index += 1;
                }
            }

            // Return selected talks
            selected
        };

        let mut tracks = Vec::new();
        let mut talks = self.talks.clone();

        while talks.len() > 0 {
            let mut track = Track::new();

            track.morning = inner_scheduler(&mut talks, MORNING_START, MORNING_END);
            track.morning.push(Talk::as_lunch());

            track.afternoon = inner_scheduler(&mut talks, AFTERNOON_START, AFTERNOON_END);
            track.afternoon.push(Talk::as_networking());

            tracks.push(track);
        }

        tracks
    }
}
