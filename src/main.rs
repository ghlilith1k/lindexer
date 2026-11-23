// lindexer, a CLI file by-type sorter/indexer (soon with online capabilities), written in Rust.
// Copyright (C) 2026 ghlilith1k

// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

// dependencies
use std::io;
use std::io::Write;
use std::fs;
use std::fs::OpenOptions;
use std::fs::File;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use glob::glob;
use crossterm::{event::{read, Event, KeyCode}, terminal};
use chrono::{Local, Timelike};

// constants
const CONFIG_FILE: &str = ".config/lindexer/lindexer.toml";
const CONFIG_DIR: &str = ".config/lindexer";
const LOG_FILE: &str = ".config/lindexer/logs.txt";

// main thread
fn main() -> Result<(), std::io::Error>  {
    let mut index = Log {
        log_index: "".to_string()
    };
    let config = if Config::validate(CONFIG_FILE) {
        Config::parse(CONFIG_FILE.to_string())
    } else {
        Config::create(CONFIG_DIR, CONFIG_FILE);
        return Ok(())
    };
    let logcheck = config.logging;
    match terminal::enable_raw_mode() {
        Ok(_) => {
            println!("q = quit, c = check arguments\r\n")
        }
        Err(error) => {
            println!("couldn't enable raw mode: {}\r\n", error);
        }
    }
    loop {
        if let Event::Key(key) = read()? {
            if key.code == KeyCode::Char('q') {
                if logcheck == true {
                    match Log::writelog(LOG_FILE, &index) {
                        Ok(_) => {
                            println!("session logged successfully\r\n")
                        }
                        Err(error) => {
                            println!("couldn't log session: {}\r\n", error)
                        }
                    }
                }
                break
            }
            if key.code == KeyCode::Char('c') {
                println!("logging: {}\r\n", config.logging)
            }
        }
    }
    match terminal::disable_raw_mode() {
        Ok(_) => {}
        Err(error) => {
            println!("couldn't disable raw mode: {}", error)
        }
    }
    println!("exiting");
    Log::writelog(LOG_FILE, &index)
}

// config implementations
pub struct Config {
    pub logging: bool
}
impl Config {
    
    // config parser
    fn parse(path: String) -> Config {
        let home = env::var_os("HOME").expect("HOME not set");
        let config_file = PathBuf::from(home).join(path);
        match fs::read_to_string(config_file) {
            Ok(config) => {
                for line in config.lines() {
                    let Some((variable, value)) = line.split_once('=') else {
                        continue
                    };
                    match variable.trim() {
                        "logging" => {
                            if value.trim() == "true" {
                                return Config { logging: true }
                            }
                        }
                        _ => {}
                    }
                }
                return Config { logging: false }
            }
            Err(error) => { 
                println!("couldn't read config file contents: {}\r\n", error);
                return Config { logging: false }
                
            }
        }
    }
    
    // config validator
    fn validate(path: &str) -> bool {
        let home = env::var_os("HOME").expect("HOME not set");
        let config_file = PathBuf::from(home).join(path);
        Path::new(&config_file).is_file()
    }
    
    // config creators
    fn create(dir: &str, file: &str) {
        let home = env::var_os("HOME").expect("HOME not set");
        let config_dir = PathBuf::from(&home).join(dir);
        let config_file = PathBuf::from(home).join(file);
        loop {
            if config_dir.is_dir() {
                match File::create(&config_file) {
                    Ok(_) => {
                        println!("successfully created config file, relaunch lindexer\r\n");
                        break
                    }
                    Err(error) => {
                        println!("couldn't create config file: {}\r\n", error);
                        break
                    }
                }
            } else {
                match fs::create_dir_all(&config_dir) {
                    Ok(_) => {
                        println!("successfully create config directory\r\n")
                    }
                    Err(error) => {
                        println!("couldn't create config directory: {}\r\n", error);
                        break
                    }
                }
            }
        }
    }
}

// logging
#[derive(Debug)]
struct Log {
    log_index: String
}
impl Log {
    // adds log
    fn printlog(&mut self, message: String, logging: bool) {
    let now = Local::now();
    let message = format!("[{:02}:{:02}:{:02}] {}", now.hour(), now.minute(), now.second(), message);
    println!("{}\r\n", message);
    if logging {
        self.log_index.push_str(&message);
    }
    }
    // writes logs to file
    fn writelog(file: &str, strings: &Log) -> std::io::Result<()> {
        let home = env::var_os("HOME").expect("HOME not set");
        let target = PathBuf::from(home).join(file);
        let mut final_target = OpenOptions::new().create(true).append(true).open(target)?;
        final_target.write_all(strings.log_index.as_bytes())
    }
}