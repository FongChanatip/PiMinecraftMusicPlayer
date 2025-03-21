use crate::external_factors;

use chrono::DateTime;
use chrono_tz::Tz;
use external_factors::get_time;
use std::{env, fmt::format, process::Command};

use super::random_time_generator::{get_weekend_time, get_weekday_time};

const RESET_COMMAND: &str = "crontab -r";
const SCHEDULER_TIME: &str = "0 0 * * *";

fn sample(occurance: i8) -> Vec<f64> {
    
    let mut times: Vec<f64> = Vec::new();

    let time = get_time::get_pacific_dt();

    for _i in 0..occurance {
        if get_time::is_weekend(time) {
            times.push(get_weekend_time());
        } else {
            times.push(get_weekday_time());
        }

    }

    return times
    
}

pub fn schedule_cron() -> bool {
    let times = sample(12);
    let mut cron_time: Vec<String> = Vec::new();
    let cur_time = get_time::get_pacific_dt();
    for t in times{
        cron_time.push(time_to_cron(cur_time, t));
    }

    add_to_crontab(cron_time)
}

fn add_to_crontab(cron_time: Vec<String>) -> bool {
    
    run_command(&RESET_COMMAND.to_string());

    let scheduler_cron = get_add_cron_job_command(&SCHEDULER_TIME.to_string(), &"0".to_string());
    if !run_command(&scheduler_cron.to_string()){
        return false;
    }

    let mut success = true;

    for t in cron_time{
        let cmd = get_add_cron_job_command(&t, &"1".to_string());
        success |= run_command(&cmd);
    }

    let keep_alive = get_keep_alive_cmd();
    success |= run_command(&keep_alive);

    return success;

}

fn get_add_cron_job_command(cron_formatted_time: &String, arg: &String) -> String{

    let cron_command = format!(r#"(crontab -l ; echo "{} ~/MinecraftMusicPlayer.sh {}") | crontab -"#, cron_formatted_time, arg);

    cron_command

}

fn run_command(command: &String) -> bool {
    let status = Command::new("sh")
        .arg("-c")
        .arg(command)
        .status().unwrap();
    return status.success();
}

fn time_to_cron(cur_time: DateTime<Tz>, time: f64) -> String{
    
    let day = get_time::get_pacific_day(cur_time);
    let month = get_time::get_pacific_month(cur_time);
    let hour: i8 = time as i8;
    let min: i8 = ((time - (hour as f64)) * 60.0) as i8;

    let cron_str = format!("{} {} {} {} *", min, hour, day, month);

    return cron_str;
}

fn get_keep_alive_cmd() -> String {
    let mpg123: String = env::var("MPG123_PATH")
        .expect("SONG_JSON_PATH must be set.")
        .parse()
        .unwrap();

    let silence_path: String = env::var("SILENCE_PATH")
        .expect("SONG_JSON_PATH must be set.")
        .parse()
        .unwrap();

    format!(r#"(crontab -l ; echo "*/5 * * * * {} -o pulse '{}' > ~/keep-alive-log.txt 2>&1") | crontab -"#, mpg123, silence_path)
}