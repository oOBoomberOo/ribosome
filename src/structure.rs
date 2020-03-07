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
	#[serde(rename = "Properites")]
	properties: Option<Vec<nbt::Value>>
}
#[derive(Debug, Deserialize)]
struct Block {
	state: usize,
	pos: [i32; 3],
	nbt: Option<nbt::Value>
}

use crate::{Config, config::Mode};
impl Structure {
	pub fn compile(&self, config: &Config) -> String {
		let mut result = vec![
			format!("scoreboard players set {} {} 1", config.scoreboard, config.objective)
		];

		for block in &self.blocks {
			let palette = self.palette[block.state].clone();
			let [x, y, z] = self.calculate_pos(block.pos, config);
			// TODO: Handle NBT data
			let _nbt = block.nbt.clone();

			let block = palette.name;
			if config.void && block == "minecraft:air" {
				continue;
			}

			let line = format!("execute if score #structure.pass ffi.ribosome matches 1 unless block ~{} ~{} ~{} {} run scoreboard players set #structure.pass ffi.ribosome 0", x, y, z, block);

			result.push(line);
		}

		result.join("\n")
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