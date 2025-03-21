use super::song_picker::get_best_song;
use dotenv::dotenv;
use std::path::Path;
use std::process::ExitStatus;
use std::{env, process::Command};

pub async fn play_song() {
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

pub fn play_mp3(path: &String) {
    let mpg123: String = env::var("MPG123_PATH")
        .expect("SONG_JSON_PATH must be set.")
        .parse()
        .unwrap();

    let mut cmd = Command::new(mpg123);

    if let Ok(xdg_runtime_dir) = env::var("XDG_RUNTIME_DIR") {
        cmd.env("XDG_RUNTIME_DIR", xdg_runtime_dir);
    }

    let output = cmd
        .arg("-o")
        .arg("pulse")
        .arg(path)
        .output()
        .expect("Failed to execute mpg123");

    if !output.status.success() {
        eprintln!("mpg123 exited with status: {}", output.status);
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    } else {
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    }
}
