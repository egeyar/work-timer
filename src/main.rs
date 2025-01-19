use chrono::{DateTime, NaiveTime, TimeDelta, Utc};
use std::env;
use std::fs;
use std::io::BufRead;
use std::path::{Path, PathBuf};

fn main() {
    println!("Hello from the Work Timer!");

    let path_buf = get_todays_path();
    let todays_path = path_buf.as_path();

    let mut start_time: Option<NaiveTime> = None;
    let mut total_worked = TimeDelta::zero();

    match todays_path.try_exists() {
        Ok(true) => {
            // Read times
            for line in fs::read_to_string(todays_path).unwrap().lines() {
                let time = NaiveTime::parse_from_str(line, "%H:%M:%S").unwrap();
                compute_work_times(&mut start_time, &time, line, &mut total_worked);
            }
        }
        Ok(false) => {
            println!("It's a new dawn, it's a new day...");
            std::fs::File::create(todays_path).unwrap();
        }
        Err(e) => {
            panic!("Cannot check if file exists: {:?}", e);
        }
    }

    let mut line = String::new();
    let stdin = std::io::stdin();
    loop {
        stdin.lock().read_line(&mut line).unwrap();
        let now = Utc::now().time();
        let now_str = now.format("%H:%M:%S").to_string();
        compute_work_times(&mut start_time, &now, &now_str, &mut total_worked);
        write_to_file(todays_path, &now_str);
    }
}

fn get_todays_path() -> PathBuf {
    #[allow(deprecated)]
    let mut path_buf = match env::home_dir() {
        Some(buf) => {
            println!("Using home directory: {}", buf.display());
            buf
        }
        None => panic!("Impossible to get your home dir!"),
    };

    path_buf.push(".work_timer/");
    fs::create_dir_all(path_buf.as_path()).expect("Cannot create dir");

    let dt: DateTime<Utc> = Utc::now();
    let file_name = dt.date_naive().format("%Y-%m-%d.txt").to_string();

    path_buf.push(file_name);
    path_buf
}

fn write_to_file(path: &Path, line: &str) {
    use std::fs::OpenOptions;
    use std::io::prelude::*;

    let mut file = OpenOptions::new().append(true).open(path).unwrap();

    writeln!(file, "{}", line).unwrap();
    file.flush().unwrap();
}

fn format_time_delta(td: &TimeDelta) -> String {
    let hours = td.num_hours();
    let minutes = td.num_minutes() - (hours * 60);
    format!("{:02}:{:02}", hours, minutes)
}

fn compute_work_times(
    start_time: &mut Option<NaiveTime>,
    new_time: &NaiveTime,
    new_time_str: &str,
    total_worked: &mut TimeDelta,
) {
    match start_time {
        Some(stime) => {
            println!("Stopped working at {}", new_time_str);
            let worked = *new_time - *stime;
            println!("Worked {}", format_time_delta(&worked));
            *total_worked += worked;
            println!("Total work time today: {}", format_time_delta(total_worked));
            *start_time = None;
        }
        None => {
            println!("Starting to work at {}", new_time_str);
            *start_time = Some(*new_time);
        }
    }
}
