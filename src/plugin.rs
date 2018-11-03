use samp_sdk::consts::*;
use samp_sdk::types::Cell;
use samp_sdk::amx::AMX;
use eliza::Eliza;
use natives::Natives;

define_native!(create,path_to_script:String);
define_native!(response,bot:usize,query:String,string:ref Cell,size:usize);


pub struct ChatBot{
	pub bots:Vec<Eliza>,
}

impl ChatBot {
	pub fn load(&self) -> bool {
		log!("[ChatBotPlugin] Plugin started");
		return true;
	}

	pub fn unload(&self) {
		 log!("[ChatBotPlugin] Plugin unloaded");
	}

	pub fn amx_load(&mut self, amx: &mut AMX) -> Cell {
		let natives = natives!{
			"ChatBotCreate" => create,
			"BotResponse" => response
		};
		match amx.register(&natives) {
			Ok(_) => log!("[ChatBotPlugin] All Natives successfully loaded"),
			Err(err) => log!("[ChatBotgitPlugin] Failed to register amx instance {:?}", err),
		}

		AMX_ERR_NONE
	}

	pub fn amx_unload(&self, _amx: &AMX) -> Cell {
		AMX_ERR_NONE
	}

}

impl Default for ChatBot {
	fn default() -> Self {
		ChatBot{
			bots: Vec::new(),
		}
	}
}