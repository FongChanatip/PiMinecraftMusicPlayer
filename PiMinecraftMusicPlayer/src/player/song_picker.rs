use crate::external_factors;

use core::f32;
use dotenv::dotenv;
use external_factors::{ExternalFactors, get_external_factors};
use once_cell::sync::Lazy;
use rand::Rng;
use rand_distr::Distribution;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{env, fs, io::BufReader};

static RECENT_SONGS: Lazy<Mutex<HashMap<usize, u64>>> = Lazy::new(|| Mutex::new(HashMap::new()));

const EXCLUSION_HOURS: u64 = 6;

pub const SONGS: [&str; 54] = [
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
    "01. Ki.mp3",
    "02. Alpha.mp3",
    "03. Dead Voxel.mp3",
    "04. Blind Spots.mp3",
    "05. Flake.mp3",
    "06. Moog City 2.mp3",
    "07. Concrete Halls.mp3",
    "08. Biome Fest.mp3",
    "09. Mutation.mp3",
    "10. Haunt Muskie.mp3",
    "11. Warmth.mp3",
    "12. Floating Trees.mp3",
    "13. Aria Math.mp3",
    "14. Kyoto.mp3",
    "15. Ballad of the Cats.mp3",
    "16. Taswell.mp3",
    "17. Beginning 2.mp3",
    "18. Dreiton.mp3",
    "19. The End.mp3",
    "20. Chirp.mp3",
    "21. Wait.mp3",
    "22. Mellohi.mp3",
    "23. Stal.mp3",
    "24. Strad.mp3",
    "25. Eleven.mp3",
    "26. Ward.mp3",
    "27. Mall.mp3",
    "28. Blocks.mp3",
    "29. Far.mp3",
    "30. Intro.mp3",
];

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub struct Song {
    track: String,
    happy: f32,
    melancholic: f32,
    hopeful: f32,
    nostalgic: f32,
    mysterious: f32,
    relaxing: f32,
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

fn load_recent_songs() {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let mut recent_songs = RECENT_SONGS.lock().unwrap();
    let file_path = "recent_songs.txt";

    if let Ok(file) = File::open(file_path) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() == 2 {
                    if let (Ok(idx), Ok(timestamp)) =
                        (parts[0].parse::<usize>(), parts[1].parse::<u64>())
                    {
                        recent_songs.insert(idx, timestamp);
                    }
                }
            }
        }
    }
}

fn save_recent_songs() {
    use std::fs::File;
    use std::io::Write;

    let recent_songs = RECENT_SONGS.lock().unwrap();
    let file_path = "recent_songs.txt";

    if let Ok(mut file) = File::create(file_path) {
        for (idx, timestamp) in recent_songs.iter() {
            if let Err(e) = writeln!(file, "{},{}", idx, timestamp) {
                eprintln!("Failed to write to recent songs file: {}", e);
            }
        }
    }
}

fn record_played_song(idx: usize) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_secs();

    let mut recent_songs = RECENT_SONGS.lock().unwrap();
    recent_songs.insert(idx, now);

    recent_songs.retain(|_, &mut timestamp| now - timestamp < EXCLUSION_HOURS * 3600);

    drop(recent_songs);
    save_recent_songs();
}

fn is_recently_played(idx: usize) -> bool {
    let recent_songs = RECENT_SONGS.lock().unwrap();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_secs();

    if let Some(&timestamp) = recent_songs.get(&idx) {
        now - timestamp < EXCLUSION_HOURS * 3600
    } else {
        false
    }
}

fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

fn normalize(mut mood: MoodScores) -> MoodScores {
    let total = mood.happy
        + mood.melancholic
        + mood.hopeful
        + mood.nostalgic
        + mood.mysterious
        + mood.relaxing;
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

pub fn load_song_data() -> Vec<Song> {
    dotenv().ok();

    let path: String = env::var("SONG_JSON_PATH")
        .expect("SONG_JSON_PATH must be set.")
        .parse()
        .unwrap();

    let file = fs::File::open(path).unwrap();
    let reader = BufReader::new(file);
    let songs: Vec<Song> = serde_json::from_reader(reader).unwrap();

    if songs.len() != SONGS.len() {
        println!(
            "Warning: JSON song count ({}) doesn't match SONGS array ({})",
            songs.len(),
            SONGS.len()
        );
    }

    for (i, song) in songs.iter().enumerate() {
        if i < SONGS.len() {
            println!("Song {}: JSON='{}', Filename='{}'", i, song.track, SONGS[i]);
        }
    }

    songs
}

fn get_song_index_by_track(track: &str, songs: &[Song]) -> Option<usize> {
    for (i, song) in songs.iter().enumerate() {
        if SONGS[i].to_lowercase().contains(&song.track.to_lowercase()) {
            return Some(i);
        }
    }
    None
}

pub async fn get_best_song() -> String {
    load_recent_songs();

    let factors = get_external_factors().await;
    let cur_mood = map_factors_to_mood(factors);
    let songs_mood = load_song_data();
    let best_idx = get_min_dist_to_song_index(cur_mood, songs_mood);

    record_played_song(best_idx as usize);

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
    } else if factors.weather.short_forecast.contains("Clear")
        || factors.weather.short_forecast.contains("Sunny")
    {
        weather_mood.happy += 0.2;
        weather_mood.hopeful += 0.2;
    }

    combined_mood = sum_moods(combined_mood, normalize(weather_mood));
    count += 1;

    let time_mood = match factors.time.hour {
        5..=11 => MoodScores {
            happy: 0.5,
            hopeful: 0.5,
            ..Default::default()
        },
        12..=14 => MoodScores {
            happy: 0.4,
            relaxing: 0.6,
            ..Default::default()
        },
        15..=17 => MoodScores {
            nostalgic: 0.5,
            relaxing: 0.5,
            ..Default::default()
        },
        18..=21 => MoodScores {
            relaxing: 0.6,
            melancholic: 0.4,
            ..Default::default()
        },
        _ => MoodScores {
            mysterious: 0.7,
            melancholic: 0.3,
            ..Default::default()
        },
    };
    combined_mood = sum_moods(combined_mood, normalize(time_mood));
    count += 1;

    let season_mood = match factors.time.season.as_str() {
        "winter" => MoodScores {
            nostalgic: 1.0,
            ..Default::default()
        },
        "spring" => MoodScores {
            hopeful: 1.0,
            ..Default::default()
        },
        "summer" => MoodScores {
            happy: 1.0,
            ..Default::default()
        },
        "fall" => MoodScores {
            relaxing: 1.0,
            ..Default::default()
        },
        _ => MoodScores::default(),
    };
    combined_mood = sum_moods(combined_mood, season_mood);
    count += 1;

    let market_mood = MoodScores {
        hopeful: sigmoid(factors.market.spy),
        melancholic: 1.0 - sigmoid(factors.market.spy),
        mysterious: sigmoid(-factors.market.btc),
        happy: sigmoid(factors.market.btc),
        ..Default::default()
    };
    combined_mood = sum_moods(combined_mood, normalize(market_mood));
    count += 1;

    if factors.mercury_retrograde {
        let mercury_mood = MoodScores {
            mysterious: 0.7,
            melancholic: 0.3,
            ..Default::default()
        };
        combined_mood = sum_moods(combined_mood, mercury_mood);
        count += 1;
    }

    normalize(average_mood(combined_mood, count))
}

pub fn get_min_dist_to_song_index(current_mood: MoodScores, songs: Vec<Song>) -> i16 {
    let mut min: f32 = f32::MAX;
    let mut min_idx: usize = 0;
    let mut rng = rand::thread_rng();

    let select_first_volume = rng.gen_bool(0.4);
    let first_volume_boundary = 24;

    let mut candidates: Vec<(usize, f32)> = Vec::new();

    for i in 0..songs.len() {
        if i >= SONGS.len() {
            break;
        }

        if select_first_volume && i >= first_volume_boundary {
            continue;
        }
        if !select_first_volume && i < first_volume_boundary && rng.gen_bool(0.7) {
            continue;
        }

        let song = songs.get(i).unwrap();
        let song_mood = song_to_mood_scores(song);
        let dist = euclidean_distance(&current_mood, &song_mood);

        println!(
            "Song {} ({}): Distance = {}, Recently played = {}",
            i,
            song.track,
            dist,
            is_recently_played(i)
        );

        if is_recently_played(i) {
            continue;
        }

        candidates.push((i, dist));

        if dist < min {
            min = dist;
        }
    }

    if candidates.is_empty() {
        println!("All songs were recently played. Allowing any song.");
    }

    let threshold = min * 1.2;
    let filtered_candidates: Vec<(usize, f32)> = candidates
        .into_iter()
        .filter(|&(_, dist)| dist <= threshold)
        .collect();

    println!("Candidates within threshold: {}", filtered_candidates.len());

    if !filtered_candidates.is_empty() {
        let idx = rng.random_range(0..filtered_candidates.len());
        min_idx = filtered_candidates[idx].0;

        let song = songs.get(min_idx).unwrap();
        println!("Selected song: {} (index {})", song.track, min_idx);
    }

    save_song_selection(min_idx);

    return min_idx as i16;
}
fn save_song_selection(idx: usize) {
    use crate::external_factors::get_time;
    use std::fs::OpenOptions;
    use std::io::Write;

    let time = get_time::get_pacific_time();

    let timestamp = format!(
        "{:04}-{:02}-{:02} {:02}:{:02}",
        time.year, time.month, time.day, time.hour, time.min
    );

    let song_name = SONGS.get(idx).unwrap_or(&"Unknown");

    let log_entry = format!(
        "[{}] Selected song index: {} ({}) - Season: {}\n",
        timestamp, idx, song_name, time.season
    );

    let file_path = "song_selections.txt";
    match OpenOptions::new().create(true).append(true).open(file_path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(log_entry.as_bytes()) {
                eprintln!("Failed to write to log file: {}", e);
            }
        }
        Err(e) => eprintln!("Failed to open log file: {}", e),
    }
}

pub fn song_to_mood_scores(song: &Song) -> MoodScores {
    println!("Matching song: {}", song.track);

    MoodScores {
        happy: song.happy,
        melancholic: song.melancholic,
        hopeful: song.hopeful,
        nostalgic: song.nostalgic,
        mysterious: song.mysterious,
        relaxing: song.relaxing,
    }
}

fn euclidean_distance(mood1: &MoodScores, mood2: &MoodScores) -> f32 {
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
