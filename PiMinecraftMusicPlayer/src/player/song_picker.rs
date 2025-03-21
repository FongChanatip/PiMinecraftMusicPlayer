use crate::external_factors;

use dotenv::dotenv;
use serde::Deserialize;
use core::f32;
use rand::Rng;
use rand_distr::Distribution;
use std::{env, fs, io::BufReader};
use external_factors::{ExternalFactors, get_external_factors};

pub const SONGS: [&str; 24] = [
    "01 - Key.mp3",
    "02 - Door.mp3",
    "03 - Subwoofer Lullaby.mp3",
    "04 - Death.mp3",
    "05 - Living Mice.mp3",
    "06 - Moog City.mp3",
    "07 - Haggstrom.mp3",
    "08 - Minecraft.mp3",
    "09 - Oxygène.mp3",
    "10 - Équinoxe.mp3",
    "11 - Mice on Venus.mp3",
    "12 - Dry Hands.mp3",
    "13 - Wet Hands.mp3",
    "14 - Clark.mp3",
    "15 - Chris.mp3",
    "16 - Thirteen.mp3",
    "17 - Excuse.mp3",
    "18 - Sweden.mp3",
    "19 - Cat.mp3",
    "20 - Dog.mp3",
    "21 - Danny.mp3",
    "22 - Beginning.mp3",
    "23 - Droopy Likes Ricochet.mp3",
    "24 - Droopy Likes Your Face.mp3",
];

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

pub async fn get_best_song() -> String {
    let factors = get_external_factors().await;
    let cur_mood = map_factors_to_mood(factors);
    let songs_mood = load_song_data();
    let best_idx = get_min_dist_to_song_index(cur_mood, songs_mood);
    return SONGS.get(best_idx as usize).unwrap().to_string();
}

fn map_factors_to_mood(factors: ExternalFactors) -> MoodScores {
    let mut combined_mood = MoodScores::default();
    let mut count = 0;

    let mut weather_mood = MoodScores::default();
    weather_mood.happy += sigmoid((factors.weather.temperature as f32 - 60.0) / 10.0);
    weather_mood.nostalgic += 1.0 - sigmoid((factors.weather.temperature as f32 - 60.0) / 10.0);
    weather_mood.melancholic += factors.weather.probability_precipitation;
    weather_mood.relaxing += 1.0 - factors.weather.probability_precipitation;

    if factors.weather.is_daytime {
        weather_mood.happy += 0.3;
        weather_mood.hopeful += 0.3;
    } else {
        weather_mood.mysterious += 0.3;
        weather_mood.relaxing += 0.3;
    }
    if factors.weather.short_forecast.contains("Cloudy") {
        weather_mood.nostalgic += 0.2;
        weather_mood.melancholic += 0.2;
    } else if factors.weather.short_forecast.contains("Clear") || factors.weather.short_forecast.contains("Sunny") {
        weather_mood.happy += 0.2;
        weather_mood.hopeful += 0.2;
    }

    combined_mood = sum_moods(combined_mood, normalize(weather_mood));
    count += 1;

    // Time-of-day mood
    let time_mood = match factors.time.hour {
        5..=11 => MoodScores { happy: 0.5, hopeful: 0.5, ..Default::default() },
        12..=14 => MoodScores { happy: 0.4, relaxing: 0.6, ..Default::default() },
        15..=17 => MoodScores { nostalgic: 0.5, relaxing: 0.5, ..Default::default() },
        18..=21 => MoodScores { relaxing: 0.6, melancholic: 0.4, ..Default::default() },
        _ => MoodScores { mysterious: 0.7, melancholic: 0.3, ..Default::default() },
    };
    combined_mood = sum_moods(combined_mood, normalize(time_mood));
    count += 1;

    // Season mood
    let season_mood = match factors.time.season.as_str() {
        "winter" => MoodScores { nostalgic: 1.0, ..Default::default() },
        "spring" => MoodScores { hopeful: 1.0, ..Default::default() },
        "summer" => MoodScores { happy: 1.0, ..Default::default() },
        "fall" => MoodScores { relaxing: 1.0, ..Default::default() },
        _ => MoodScores::default(),
    };
    combined_mood = sum_moods(combined_mood, season_mood);
    count += 1;

    // Market mood
    let market_mood = MoodScores {
        hopeful: sigmoid(factors.market.spy),
        melancholic: 1.0 - sigmoid(factors.market.spy),
        mysterious: sigmoid(-factors.market.btc),
        happy: sigmoid(factors.market.btc),
        ..Default::default()
    };
    combined_mood = sum_moods(combined_mood, normalize(market_mood));
    count += 1;

    // Mercury Retrograde mood
    if factors.mercury_retrograde {
        let mercury_mood = MoodScores { mysterious: 0.7, melancholic: 0.3, ..Default::default() };
        combined_mood = sum_moods(combined_mood, mercury_mood);
        count += 1;
    }

    normalize(average_mood(combined_mood, count))
}


pub fn get_min_dist_to_song_index(current_mood: MoodScores, songs: Vec<Song>) -> i16{

    let mut min: f32 = f32::MAX;
    let mut min_idx: usize = 0;
    let mut rng = rand::rng();

    for i in 0..songs.len(){
        let song = songs.get(i).unwrap();
        let song_mood = song_to_mood_scores(song);
        let dist = euclidean_distance(&current_mood, &song_mood);
        let rand: f64 = rng.random();
        println!("{dist}");
        if dist < min{
            min = dist;
            min_idx = i;
        } else if dist == min && rand < 0.33{
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

fn sum_moods(a: MoodScores, b: MoodScores) -> MoodScores {
    MoodScores {
        happy: a.happy + b.happy,
        melancholic: a.melancholic + b.melancholic,
        hopeful: a.hopeful + b.hopeful,
        nostalgic: a.nostalgic + b.nostalgic,
        mysterious: a.mysterious + b.mysterious,
        relaxing: a.relaxing + b.relaxing,
    }
}

fn average_mood(mood: MoodScores, count: usize) -> MoodScores {
    MoodScores {
        happy: mood.happy / count as f32,
        melancholic: mood.melancholic / count as f32,
        hopeful: mood.hopeful / count as f32,
        nostalgic: mood.nostalgic / count as f32,
        mysterious: mood.mysterious / count as f32,
        relaxing: mood.relaxing / count as f32,
    }
}