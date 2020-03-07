mod config;
mod structure;
use config::Config;
use structure::Structure;

use clap::{App, Arg};
use colored::*;
use std::path::PathBuf;

fn main() {
	if let Err(error) = run() {
		eprintln!("{}", error);
	}
}

fn run() -> PResult<()> {
	let matches = App::new("Ribosome")
		.version("0.1.0")
		.author("Boomber <boomberisalreadytaken@gmail.com>")
		.about("Minecraft structure file to mcfunction converter")
		.arg(
			Arg::with_name("path")
				.short("p")
				.long("path")
				.takes_value(true)
				.required(true)
				.index(1)
				.help("Location of the structure file or directory")
				.allow_hyphen_values(true),
		)
		.arg(
			Arg::with_name("void")
				.short("v")
				.long("ignore-air")
				.help("Ignore air block inside structure file")
				.takes_value(false),
		)
		.arg(
			Arg::with_name("ignore nbt")
				.short("n")
				.long("ignore-nbt")
				.help("Ignore NBT of blocks inside structure file")
				.takes_value(false),
		)
		.arg(
			Arg::with_name("ignore block state")
				.short("b")
				.long("ignore-block-state")
				.help("Ignore Block State of blocks inside structure file")
				.takes_value(false),
		)
		.arg(
			Arg::with_name("target name")
				.short("t")
				.long("target")
				.help("Scoreboard target name, default: #structure.pass")
				.takes_value(true)
				.validator(scoreboard_validator),
		)
		.arg(
			Arg::with_name("objective")
				.short("o")
				.long("objective")
				.help("Scoreboard objective name, default: ffi.ribosome")
				.takes_value(true)
				.validator(objective_validator),
		)
		.arg(
			Arg::with_name("mode")
				.short("m")
				.long("mode")
				.help("Detection location of this structure")
				.takes_value(true)
				.possible_values(&["corner", "center", "center_top", "center_bottom"])
		)
		.get_matches();

	if let Some(path) = matches.value_of("path") {
		let mut config = Config::default();

		config.void = matches.is_present("void");
		config.ignore_nbt = matches.is_present("ignore nbt");
		config.ignore_block_state = matches.is_present("ignore block state");
		if let Some(value) = matches.value_of("target name") {
			config.set_scoreboard(value);
		};
		if let Some(value) = matches.value_of("objective") {
			config.set_objective(value);
		};
		if let Some(value) = matches.value_of("mode") {
			config.set_mode(value);
		}

		let path = PathBuf::from(path);

		if !path.exists() {
			let path = path.display().to_string().cyan();
			return Err(format!("{} does not exists", path).into());
		}

		if path.is_dir() {
			from_dir(path, &config)?;
		} else if path.is_file() {
			from_file(path, &config)?;
		}
	}

	Ok(())
}

fn scoreboard_validator(input: String) -> Result<(), String> {
	if input.len() > 40 {
		return Err(format!(
			"Target name is too long. Max is 40 but got {}",
			input.len()
		));
	}
	Ok(())
}

fn objective_validator(input: String) -> Result<(), String> {
	if input.len() > 16 {
		return Err(format!(
			"Objective name is too long. Max is 16 but got {}",
			input.len()
		));
	}
	Ok(())
}

use std::fs;
use std::fs::File;
fn from_file(path: PathBuf, config: &Config) -> PResult<()> {
	if let Some(extension) = path.extension() {
		if extension != "nbt" {
			let path = path.display().to_string().cyan();
			return Err(format!("{} is not .nbt file", path).into());
		}
	} else {
		let path = path.display().to_string().cyan();
		return Err(format!("{} is not .nbt file", path).into());
	}

	let file = File::open(&path)?;
	let structure: Structure = match nbt::from_gzip_reader(file) {
		Ok(result) => result,
		Err(error) => return Err(Error::Nbt(path, error)),
	};

	let mut output = path.clone();
	output.set_extension("mcfunction");
	let data = structure.compile(&config);
	fs::write(&output, data)?;

	println!(
		"Compiled {} to {}",
		path.display().to_string().cyan(),
		output.display().to_string().cyan()
	);

	Ok(())
}

fn from_dir(path: PathBuf, config: &Config) -> PResult<()> {
	path.read_dir()?
		.filter_map(|entry| entry.ok())
		.filter(|entry| entry.path().is_file())
		.filter(|entry| {
			if let Some(extension) = entry.path().extension() {
				if extension == "nbt" {
					return true;
				}
			}
			false
		})
		.try_for_each(|entry| from_file(entry.path(), config))?;

	Ok(())
}

type PResult<T> = Result<T, Error>;

use std::io;
#[derive(Debug)]
enum Error {
	Io(io::Error),
	Nbt(PathBuf, nbt::Error),
	Other(String),
}

use std::fmt;
impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Error::Io(error) => write!(f, "{}", error),
			Error::Nbt(path, error) => {
				write!(f, "[{}] {}", path.display().to_string().cyan(), error)
			}
			Error::Other(error) => write!(f, "{}", error),
		}
	}
}

impl From<String> for Error {
	fn from(error: String) -> Self {
		Error::Other(error)
	}
}

impl From<io::Error> for Error {
	fn from(error: io::Error) -> Self {
		Error::Io(error)
	}
}
