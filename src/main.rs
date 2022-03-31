use std::env;
use chrono::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (b, a) = get_time_from_args(&args);
    let duration = calc_duration_hour(b, a);
    println!("duration: {}", duration)
}

fn convert_datetime_from_string(s: &str) -> DateTime<Local>{
    let n: DateTime<Local> = Local.ymd(2000, 1, 1).and_hms(0, 0, 0);
    let mut iter = s.splitn(2, ":");
    let temp_hour: u32 = iter.next().unwrap().parse().unwrap();
    let minutes: u32 = iter.next().unwrap().parse().unwrap();
    let i = if temp_hour >= 24 { 1 } else { 0 };
    let hour: u32 = if temp_hour >= 24 { temp_hour - 24 } else { temp_hour };
    let dt = Local
        .ymd(n.year(), n.month(), n.day() + i)
        .and_hms(hour, minutes, 0);
    dt
}

fn get_time_from_args(args: &Vec<String>) -> (DateTime<Local>, DateTime<Local>) {
    (
        convert_datetime_from_string(&args[1]),
        convert_datetime_from_string(&args[2])
    )
}

fn calc_duration_hour(before: DateTime<Local>, after: DateTime<Local>) -> f64 {
    let duration = after - before;
    duration.num_minutes() as f64 / 60.0
}