use clap::Parser;
use colored::{self, Colorize};
use core::time;
use notify_rust::Notification;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::thread::sleep;

const MINUTE_LENGTH: f32 = 60.0;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    file: PathBuf,
    #[clap(short, long, value_parser, default_value_t = 1.0)]
    delay: f32,
    #[clap(short, long, value_parser, default_value_t = 16.0)]
    wpm: f32,
}

fn main() {
    let args = Args::parse();
    let file = args.file;
    let delay = args.delay;
    let wpm = args.wpm;
    let short_break = 5;
    let long_break = 30;
    let pomodoro_duration = 25;
    let pomodoro_long_break = 4; // after 4 pomodoro sessions -> long break

    let delta = delay / MINUTE_LENGTH;
    let wpd = wpm * delta;
    let mut minute_count: f32 = 0.0;

    let first_count = count(&file);
    let mut last_count = count(&file);

    let mut notification = Notification::new()
        .summary("Minute 0:00.00")
        .body("0 words\n0.00 wpm")
        .show()
        .unwrap();

    loop {
        sleep(time::Duration::from_secs_f32(delay));

        minute_count += delta;
        let words_count = count(&file);
        let words_delta = words_count - last_count;
        let words_total = words_count - first_count;
        let average_wpm = words_total as f32 / minute_count;
        last_count = words_count;

        println!(
            "Minute {}:{:05.2}",
            minute_count.floor(),
            (minute_count - minute_count.floor()) * MINUTE_LENGTH
        );

        if words_delta as f32 >= wpd {
            println!(
                "{} {} {}",
                "Success,".green(),
                words_delta.to_string().green(),
                "words written!".green()
            );
        } else {
            println!(
                "{} {} {}",
                "Keep your focus!".red(),
                words_delta.to_string().red(),
                "words written!".red()
            );
        }

        println!("");

        if average_wpm >= wpm {
            println!(
                "{} {:.2} {}",
                "Keep going!".green(),
                average_wpm.to_string().green(),
                "words per minute!".green()
            );
        } else {
            println!(
                "{} {:.2} {}",
                "Go go go!".red(),
                average_wpm.to_string().red(),
                "words per minute written!".red()
            );
        }

        println!("");

        println!(
            "{} {} {}",
            "Wrote".cyan(),
            words_total.to_string().cyan(),
            "words in total".cyan()
        );

        println!("");

        notification.summary(&format!(
            "Minute {}:{:05.2}",
            minute_count.floor(),
            (minute_count - minute_count.floor()) * MINUTE_LENGTH
        ));
        notification.body(&format!("{} words\n{:.2} wpm", words_total, average_wpm));
        notification.update();

        if minute_count as i32 != 0 && minute_count as i32 % pomodoro_duration == 0 {
            if minute_count as i32 % (pomodoro_duration * pomodoro_long_break) == 0 {
                println!("Nice work, it's been an hour, take a break man");
                Notification::new()
                    .summary("Nice work, it's been a while, take a break man")
                    .show()
                    .unwrap();
                take_break(long_break);
            } else {
                println!("Nice work, shake your legs! take a drink!");
                Notification::new()
                    .summary("Nice work, shake your legs! take a drink!")
                    .show()
                    .unwrap();
                take_break(short_break);
            }
        }
    }
}

fn count(file: &PathBuf) -> usize {
    return words_count::count(read_to_string(file).expect("Failed to open given file")).words;
}

fn take_break(duration: i32) {
    for n in 0..duration {
        let remaining = duration - (n + 1);
        sleep(time::Duration::from_secs(MINUTE_LENGTH as u64));
        println!("minutes remaining: {}", remaining);
    }
}
