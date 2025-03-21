use std::env;
use scheduler::scheduler::schedule_cron;
use player::play_song;

mod scheduler;
mod external_factors;
mod player;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = args[1].parse::<i32>().unwrap();;
    
    player::song_picker::load_song_data();

    if mode == 0{
        schedule_cron();
    } else {
        play_song().await;
    }
}
