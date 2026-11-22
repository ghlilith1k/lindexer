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
use std::fs;
use std::fs::File;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use glob::glob;

// constants
const CONFIG_FILE: &str = ".config/lindexer/lindexer.toml";
const CONFIG_DIR: &str = ".config/lindexer";

// main thread
fn main() {
    if Config::validate(CONFIG_FILE) {
        let config = Config::parse(CONFIG_FILE.to_string());
        println!("{}", config);
        return
    } else {
        Config::create(CONFIG_DIR, CONFIG_FILE);
        return
    }
}

// config implementations
struct Config;
impl Config {
    
    // config parser
    fn parse(path: String) -> String {
        let home = env::var_os("HOME").expect("HOME not set");
        let config_file = PathBuf::from(home).join(path);
        match fs::read_to_string(config_file) {
            Ok(config) => {
                return config.trim().to_string()
            }
            Err(error) => { 
                println!("couldn't read config file contents: {}", error);
                return "".to_string()
                
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
                        println!("successfully created config file, relaunch lindexer");
                        break
                    }
                    Err(error) => {
                        println!("couldn't create config file: {}", error);
                        break
                    }
                }
            } else {
                match fs::create_dir(&config_dir) {
                    Ok(_) => {
                        println!("successfully create config directory")
                    }
                    Err(error) => {
                        println!("couldn't create config directory: {}", error);
                        break
                    }
                }
            }
        }
    }
}