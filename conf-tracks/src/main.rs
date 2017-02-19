#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate chrono;
extern crate clap;

mod parse;
mod talks;
mod scheduler;
mod tracks;

use clap::{Arg, App};

use scheduler::{Scheduler};


fn main() {
    let opts = App::new("conf-traks")
        .arg(Arg::with_name("input")
            .required(true)
            .index(1)
            .help("Sets a custom config file"))
        .get_matches();

    let talks = parse::from_file(opts.value_of("input").unwrap());
    let mut scheduler = Scheduler::from_talks(talks);
    let tracks = scheduler.schedule();

    for (index, track) in tracks.iter().enumerate() {
        println!("Track #{}", index + 1);
        println!("{}", track);
    }
}
