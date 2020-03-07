use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Structure {
	#[serde(rename = "DataVersion")]
	data_version: i32,
	size: [i32; 3],
	palette: Vec<Palette>,
	blocks: Vec<Block>
}

#[derive(Debug, Deserialize, Clone)]
struct Palette {
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Properties")]
	properties: Option<nbt::Value>
}
#[derive(Debug, Deserialize)]
struct Block {
	state: usize,
	pos: [i32; 3],
	nbt: Option<nbt::Value>
}

use nbt::Value;
use crate::{Config, config::Mode};
impl Structure {
	pub fn compile(&self, config: &Config) -> String {
		let mut result = vec![
			format!("scoreboard players set {} {} 1", config.scoreboard, config.objective)
		];

		for block in &self.blocks {
			let palette: Palette = self.palette[block.state].clone();
			let [x, y, z] = self.calculate_pos(block.pos, config);

			let block_state = match palette.properties {
				Some(states) if !config.ignore_block_state => self.compile_block_state(&states),
				_ => String::default()
			};
			let nbt = match block.nbt.clone() {
				Some(nbt) if !config.ignore_nbt => self.compile_nbt(&nbt),
				_ => String::default()
			};

			let block = palette.name;
			if config.void && block == "minecraft:air" {
				continue;
			}

			let line = format!("execute if score {target} {objective} matches 1 unless block ~{x} ~{y} ~{z} {block_id}{block_state}{nbt} run scoreboard players set {target} {objective} 0", x = x, y = y, z = z, block_id = block, block_state = block_state, nbt = nbt, target = config.scoreboard, objective = config.objective);

			result.push(line);
		}

		result.join("\n")
	}

	fn compile_nbt(&self, nbt: &Value) -> String {
		match nbt {
			Value::Byte(value) => format!("{}b", value),
			Value::ByteArray(value) => {
				let byte_array: Vec<String> = value.iter().map(|x| format!("{}b", x)).collect();
				let inner = byte_array.join(", ");
				format!("[{}]", inner)
			},
			Value::Compound(value) => {
				let compound: Vec<String> = value.iter().map(|(key, value)| format!("{}: {}", key, self.compile_nbt(value))).collect();
				let inner = compound.join(", ");
				format!("{{{}}}", inner)
			},
			Value::Double(value) => format!("{}d", value),
			Value::Float(value) => format!("{}f", value),
			Value::Int(value) => format!("{}", value),
			Value::IntArray(value) => {
				let int_array: Vec<String> = value.iter().map(|x| x.to_string()).collect();
				let inner = int_array.join(", ");
				format!("[{}]", inner)
			},
			Value::List(value) => {
				let list: Vec<String> = value.iter().map(|value| self.compile_nbt(value)).collect();
				let inner = list.join(", ");
				format!("[{}]", inner)
			},
			Value::Long(value) => format!("{}L", value),
			Value::LongArray(value) => {
				let long_array: Vec<String> = value.iter().map(|x| format!("{}L", x)).collect();
				let inner = long_array.join(", ");
				format!("[{}]", inner)
			},
			Value::Short(value) => format!("{}s", value),
			Value::String(value) => format!("\"{}\"", value),
		}
	}

	fn compile_block_state(&self, states: &Value) -> String {
		if let Value::Compound(states) = states {	
			let mut inner = Vec::default();
			for (key, name) in states {
				let value = format!("{}={}", key, name);
				inner.push(value);
			}
			let inner = inner.join(", ");
			format!("[{}]", inner)
		}
		// If Value is not Compound then it isn't block state
		else {
			String::default()
		}
	}

	fn calculate_pos(&self, pos: [i32; 3], config: &Config) -> [i32; 3] {
		let [dx, dy, dz] = self.size;
		let [dx, dy, dz] = match config.mode {
			Mode::Corner => [0, 0, 0],
			Mode::Center => [dx / 2, dy / 2, dz / 2],
			Mode::CenterTop => [dx / 2, dy, dz / 2],
			Mode::CenterBottom => [dx / 2, 0, dz / 2],
		};

		let [x, y, z] = pos;
		[x - dx, y - dy, z - dz]
	}
}