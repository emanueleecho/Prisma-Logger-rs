use chrono::Local;
use colored::*;

pub struct Logger {
    website: String,
}

impl Logger {
    pub fn new(website: &str) -> Self {
        Logger {
            website: website.to_string(),
        }
    }

    fn current_time() -> String {
        let now = Local::now();
        now.format("%H:%M:%S.%3f").to_string()
    }

    pub fn log(&self, msg: &str) {
        let time = Self::current_time();
        println!(
            "{} | [{}] : {}",
            time.white(),
            self.website.cyan(),
            msg.white()
        );
    }

    pub fn blue(&self, msg: &str) {
        let time = Self::current_time();
        println!(
            "{} | [{}] : {}",
            time.white(),
            self.website.blue(),
            msg.blue()
        );
    }

    pub fn red(&self, msg: &str) {
        let time = Self::current_time();
        println!(
            "{} | [{}] : {}",
            time.white(),
            self.website.red(),
            msg.red()
        );
    }

    pub fn green(&self, msg: &str) {
        let time = Self::current_time();
        println!(
            "{} | [{}] : {}",
            time.white(),
            self.website.green(),
            msg.green()
        );
    }

    pub fn yellow(&self, msg: &str) {
        let time = Self::current_time();
        println!(
            "{} | [{}] : {}",
            time.white(),
            self.website.yellow(),
            msg.yellow()
        );
    }

    pub fn error(&self, msg: &str) {
        let time = Self::current_time();
        eprintln!(
            "{} | [{}] : {}",
            time.white(),
            self.website.red(),
            msg.red().bold()
        );
    }
}
