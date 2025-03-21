use player::play_song;
use scheduler::scheduler::schedule_cron;
use std::env;

mod external_factors;
mod player;
mod scheduler;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = args[1].parse::<i32>().unwrap();

    player::song_picker::load_song_data();

    if mode == 0 {
        schedule_cron();
    } else {
        play_song().await;
    }
}
