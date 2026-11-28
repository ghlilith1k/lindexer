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
use std::{
    io::{Write},
    fs,
    fs::{OpenOptions,File},
    env,
    path::{Path,PathBuf}
};
use crossterm::{
    event::{read, Event, KeyCode},
    terminal
};
use chrono::{Local, Timelike};
use glob::glob;

// constants
const CONFIG_FILE: &str = ".config/lindexer/config.conf";
const CONFIG_DIR: &str = ".config/lindexer";
const LOG_FILE: &str = ".config/lindexer/logs.txt";

// main thread
fn main() -> Result<(), std::io::Error>  {
    
    // all the parsing stuff
    let mut index = Log {
        log_index: "".to_string()
    };
    let config = if Config::validate(CONFIG_FILE) {
        Config::parse(CONFIG_FILE.to_string())
    } else {
        Config::create(CONFIG_DIR, CONFIG_FILE);
        return Ok(())
    };
    let logging = config.logging;
    let mut sortdir = config.sortdir;
    if sortdir == "".to_string() {
        sortdir = "none".to_string()
    }

    // ----------THE ACTUAL CODE----------
    Input::start();
    println!("lindexer v0.0.2-alpha\r\nCopyright (C) 2026 ghlilith1k\r\ncheck https://github.com/ghlilith1k/lindexer for details\r\n\nq: quit, h: help, c: check config, s: sort\r\n");
    loop {
        match Input::check() {
            'q' => { break }
            'h' => { println!("index:\r\n\x20 quit (q): terminates the program\r\n\x20 help (h): prints this message\r\n\x20 check config (c): checks config variables applied at runtime\r\n\x20 sort (s): sorts files in specified directory\r\n") }
            'c' => { 
                println!("logging = {}, sortdir = {}\r\n", logging, sortdir);
                if sortdir == "none" {
                    println!("no sortdir defined, please define it in ~/.config/lindexer/config.conf\r\nremember, it has to be inside another directory\r\n");
                    break
                }
            }
            's' => { Sort::sort(&sortdir) }
            _ => {}
        }
    }
    // ----------END----------

    Input::stop();
    println!("exiting");
    Log::writelog(LOG_FILE, &index)
    
}

// config implementations
pub struct Config {
    pub logging: bool,
    pub sortdir: String
}
impl Config {
    
    // config parser
    fn parse(path: String) -> Config {
        let mut logging = false;
        let mut sortdir = String::new();
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
                                logging = true
                            }
                        }
                        "sortdir" => {
                            sortdir.push_str(value)
                        } 
                        _ => {}
                    }
                }
            }
            Err(error) => { 
                println!("couldn't read config file contents: {}\r\n", error);
            }
        }
        return Config { logging: logging, sortdir: sortdir }
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
    log_index: String,
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

struct Input {}
impl Input {
    
    // starts raw mode
    fn start() {
        match terminal::enable_raw_mode() {
            Ok(_) => {}
            Err(error) => { println!("failed to enable raw mode: {}", error); return }
        }
    }
    
    // stops raw mode
    fn stop() {
        match terminal::disable_raw_mode() {
            Ok(_) => {}
            Err(error) => { println!("failed to disable raw mode: {}", error); return }
        }
    }
    
    // checks current char
    fn check() -> char {
        if let Ok(Event::Key(current)) = read() {
            if current.code == KeyCode::Char('q') {
                return 'q'
            } else if current.code == KeyCode::Char('h') {
                return 'h'
            } else if current.code == KeyCode::Char('c') {
                return 'c'
            } else if current.code == KeyCode::Char('s') {
                return 's'
            } else { return '_' }
        } else {
            return '_'
        }
    }

}

// sorting
struct Sort {}
impl Sort {

    fn sort(directory: &str) {
        if Path::new(directory.trim()).is_dir() {
            let sortdir = format!("{}/*", directory);
            for entry in glob(&sortdir).expect("could not get entries") {
                let entry = entry.unwrap();
                let file = entry.to_string_lossy();
                if file.contains(".mp3") {
                    println!("found mp3 file: {}", file)
                } else {
                    println!("hmmm")
                }
            }
        } else {
            println!("{} directory not found\r\n", directory)
        }
    }

}
