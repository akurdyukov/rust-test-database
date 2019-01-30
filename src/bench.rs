use std::time::{SystemTime, UNIX_EPOCH};

pub struct Bench {
    name: String,
    timestamp: u128,
}

impl Bench {
    pub fn new(name: &str) -> Self {
        Bench {
            name: name.to_string(),
            timestamp: Bench::cur_timestamp_mils(),
        }
    }

    pub fn cur_timestamp_mils() -> u128 {
        let system_time = SystemTime::now();
        let since_the_epoch = system_time.duration_since(UNIX_EPOCH).unwrap();
        let mils = since_the_epoch.as_millis();
        // println!("{}", mils);
        mils
        // SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
    }
}

impl Drop for Bench {
    fn drop(&mut self) {
        println!("{}: {} mils", self.name, Bench::cur_timestamp_mils() - self.timestamp);
    }
}