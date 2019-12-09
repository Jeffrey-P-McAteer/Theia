
use app_dirs::{AppDataType};
use serde::{Serialize, Deserialize};
use toml;

use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;

use crate::APP_INFO;

pub fn print_config_files() {
	println!("Config file: {}", get_config_file().to_string_lossy());
	println!("Cache directory: {}", get_cache_dir().to_string_lossy());
}

pub fn get_config() -> Config {
	let config_f = get_config_file();
	if config_f.as_path().exists() {
		let mut s = String::new();
		File::open(config_f).unwrap().read_to_string(&mut s).unwrap();
		return toml::from_str(&s).unwrap();
	}
	else {
		Config {
			name: "Words".to_string()
		}
	}
}

pub fn get_config_file() -> PathBuf {
	if let Ok(mut config_dir) = app_dirs::get_app_root(AppDataType::UserConfig, &APP_INFO) {
		config_dir.push("config.toml");
		return config_dir;
	}
	else {
		panic!("Error: this OS prevents {} from using a config directory.", APP_INFO.name);
	}
}

pub fn get_cache_dir() -> PathBuf {
	if let Ok(config_dir) = app_dirs::get_app_root(AppDataType::UserCache, &APP_INFO) {
		return config_dir;
	}
	else {
		panic!("Error: this OS prevents {} from using a cache directory.", APP_INFO.name);
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
	pub name: String
}

