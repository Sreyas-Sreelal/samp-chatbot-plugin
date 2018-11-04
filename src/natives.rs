use samp_sdk::types::Cell;
use samp_sdk::amx::{AmxResult, AMX};
use eliza::Eliza;

pub trait Natives {
		fn create(&mut self,_:&AMX,path_to_script:String) -> AmxResult<Cell>;
		fn response(&mut self,_:&AMX,bot:usize,query:String, string:&mut Cell,size:usize) -> AmxResult<Cell>;
}

impl Natives for super::ChatBot{

	/*
		Native: ChatBotCreate
		
		Description:
			Creates an eliza bot instance and push the instance to global vector
		
		Parameters:
			- `path[]`: string containing path to bot script file
		
		Returns:
			- `0 .. etc`: Pushed instance' index
			- `-1`: If failed to create an instance

	*/
	fn create(&mut self,_:&AMX,path_to_script:String) -> AmxResult<Cell>{
		let full_path = "scripts/".to_owned() + path_to_script.as_str();
		match Eliza::new(full_path.as_str()) {
			Ok(bot) => {
				self.bots.push(bot);
				Ok(self.bots.len() as Cell - 1)
			},
			Err(err) => {
				log!("[ChatBotPlugin] failed to load instance of bot {:?}",err);
				Ok(-1)
			},
		}
	}

	/*
		Native: BotResponse
		
		Description:
			Generates a response from the query send to the bot
		
		Parameters:
			- `ChatBot:bot`: Bot instance id
			- `query[]`: Message to send to user
			- `string[]`: String to store response from bot
			- `size`: size of the destination string
		
		Returns: 1

	*/
	fn response(&mut self,_:&AMX,bot:usize,query:String, string:&mut Cell,size:usize) -> AmxResult<Cell>{
		let encoded = samp_sdk::cp1251::encode(self.bots[bot].respond(query.as_str()).as_str())?;
		set_string!(encoded,string,size);
		Ok(1)
	}
}


