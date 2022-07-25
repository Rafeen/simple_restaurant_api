use rand::Rng;

async fn generate_random_item_duration() -> f64 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(5..=15) as f64;
}
