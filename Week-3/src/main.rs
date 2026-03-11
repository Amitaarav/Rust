use chrono::{Utc,Local};
use dotenv::dotenv;
use std::env;

fn main() {
    let utc = Utc::now();
    let local_time = Local::now();
    print!("{}", utc);
    print!("{}", local_time);

    // ================
    dotenv().ok();
    let redis_url_result = env::var("REDIS_ADDRESS");
    let redis_url= redis_url_result.unwrap();
    // ==========
    // unwrap
}

