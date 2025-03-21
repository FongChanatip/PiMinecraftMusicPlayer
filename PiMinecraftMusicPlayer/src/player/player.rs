use dotenv::dotenv;
use std::{env, process::Command};
use std::path::Path;
use super::song_picker::get_best_song;

pub async fn play_song(){
    dotenv().ok();

    let album_path_str: String = env::var("ALBUM_PATH")
        .expect("SONG_JSON_PATH must be set.")
        .parse()
        .unwrap();
    let path = Path::new(&album_path_str);
    
    let best_song = get_best_song().await;
    let song_path = path.join(best_song).to_str().unwrap().to_string();

    play_mp3(&song_path);
}

pub fn play_mp3(path: &String){
    Command::new("mpg123")
        .arg("-v")
        .arg(path)
        .status()
        .expect("failed to play song");
}