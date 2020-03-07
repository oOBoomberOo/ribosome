#[derive(Debug, Eq, PartialEq)]
pub struct Config {
	pub void: bool,
	pub scoreboard: String,
	pub objective: String,
	pub mode: Mode,
	pub ignore_nbt: bool,
	pub ignore_block_state: bool
}

impl Default for Config {
	fn default() -> Self {
		Config {
			void: false,
			scoreboard: "#structure.pass".to_string(),
			objective: "ffi.ribosome".to_string(),
			mode: Mode::default(),
			ignore_nbt: false,
			ignore_block_state: false
		}
	}
}

impl Config {
	pub fn set_scoreboard(&mut self, input: &str) {
		self.scoreboard = input.to_owned();
	}

	pub fn set_objective(&mut self, input: &str) {
		self.objective = input.to_owned();
	}

	pub fn set_mode(&mut self, input: &str) {
		self.mode = Config::get_mode(input);
	}

	fn get_mode(input: &str) -> Mode {
		match input {
			"corner" => Mode::Corner,
			"center" => Mode::Center,
			"center_top" => Mode::CenterTop,
			"center_bottom" => Mode::CenterBottom,
			_ => Mode::default()
		}
	}
}

#[derive(Eq, PartialEq, Debug)]
pub enum Mode {
	Corner,
	Center,
	CenterTop,
	CenterBottom
}

impl Default for Mode {
	fn default() -> Self {
		Mode::Corner
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn get_corner_mode() {
		assert_eq!(Config::get_mode("corner"), Mode::Corner);
	}
	#[test]
	fn get_center_mode() {
		assert_eq!(Config::get_mode("center"), Mode::Center);
	}
	#[test]
	fn get_center_top_mode() {
		assert_eq!(Config::get_mode("center_top"), Mode::CenterTop);
	}
	#[test]
	fn get_center_bottom_mode() {
		assert_eq!(Config::get_mode("center_bottom"), Mode::CenterBottom);
	}
	#[test]
	#[should_panic]
	fn get_wrong_center_mode() {
		assert_eq!(Config::get_mode("center"), Mode::CenterBottom);
	}
	#[test]
	fn get_random_mode() {
		assert_eq!(Config::get_mode("megumin"), Mode::Corner);
	}
}