use rand::Rng;
use rand_distr::{Distribution, Normal};

const WEEKEND_DIST: [f64; 4] = [12.5,20.0,2.6,2.7]; // m0, m1, s0, s1
const WEEKDAY_DIST: [f64; 4] = [11.5,18.5,2.0,2.0]; // m0, m1, s0, s1

pub fn get_weekday_time() -> f64{
    return get_time(WEEKDAY_DIST);
}

pub fn get_weekend_time() -> f64{
    return get_time(WEEKEND_DIST);
}

fn get_time(dist_config: [f64; 4]) -> f64{
    return generate_bimodal(dist_config[0], dist_config[1], dist_config[2], dist_config[3]) % 24.0;
}

fn generate_bimodal(m0: f64, m1: f64, s0: f64, s1: f64) -> f64 {
    let mut rng = rand::rng();
    let x: f64 = rng.random();
    if x < 0.5 {
        Normal::new(m0, s0).unwrap().sample(&mut rng)
    } else {
        Normal::new(m1, s1).unwrap().sample(&mut rng)
    }
}