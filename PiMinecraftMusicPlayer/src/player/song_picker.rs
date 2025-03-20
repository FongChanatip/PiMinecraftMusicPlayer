use dotenv::dotenv;
use serde::Deserialize;
use core::f32;
use std::{env, fs, io::BufReader, ops::Deref};

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub struct Song{
    track: String,
    happy: f32,
    melancholic: f32,
    hopeful: f32,
    nostalgic: f32,
    mysterious: f32,
    relaxing: f32
}

#[derive(Debug, Default)]
struct MoodScores {
    happy: f32,
    melancholic: f32,
    hopeful: f32,
    nostalgic: f32,
    mysterious: f32,
    relaxing: f32,
}


fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

fn normalize(mut mood: MoodScores) -> MoodScores {
    let total = mood.happy + mood.melancholic + mood.hopeful + mood.nostalgic + mood.mysterious + mood.relaxing;
    if total != 0.0 {
        mood.happy /= total;
        mood.melancholic /= total;
        mood.hopeful /= total;
        mood.nostalgic /= total;
        mood.mysterious /= total;
        mood.relaxing /= total;
    }
    mood
}

pub fn load_song_data() -> Vec<Song>{
    dotenv().ok();

    let path: String = env::var("SONG_JSON_PATH")
        .expect("SONG_JSON_PATH must be set.")
        .parse()
        .unwrap();


    let file = fs::File::open(path).unwrap();
    let reader = BufReader::new(file);
    let songs: Vec<Song> = serde_json::from_reader(reader).unwrap();

    songs
}

pub fn map_factors_to_mood(){
    // TODO
}

pub fn get_min_dist_to_song_index(current_mood: MoodScores, songs: Vec<Song>) -> i16{

    let mut min: f32 = f32::MIN;
    let mut min_idx: usize = 0;

    for i in 0..songs.len(){
        let song = songs.get(i).unwrap();
        let song_mood = song_to_mood_scores(song);
        let dist = euclidean_distance(&current_mood, &song_mood);
        if dist < min{
            min = dist;
            min_idx = i;
        }
    }

    return min_idx as i16;

}

pub fn song_to_mood_scores(song: &Song) -> MoodScores{
    MoodScores { 
        happy: song.happy, 
        melancholic: song.melancholic, 
        hopeful: song.hopeful, 
        nostalgic: song.nostalgic, 
        mysterious: song.mysterious, 
        relaxing: song.relaxing, ..Default::default()
    }
}

fn euclidean_distance(mood1: &MoodScores, mood2: &MoodScores) -> f32{

    let mut sum: f32 = 0.0;

    sum += (mood1.happy - mood2.happy).powf(2.0);
    sum += (mood1.melancholic - mood2.melancholic).powf(2.0);
    sum += (mood1.hopeful - mood2.hopeful).powf(2.0);
    sum += (mood1.nostalgic - mood2.nostalgic).powf(2.0);
    sum += (mood1.mysterious - mood2.mysterious).powf(2.0);
    sum += (mood1.relaxing - mood2.relaxing).powf(2.0);

    sum.sqrt()
    
}