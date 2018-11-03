use samp_sdk::consts::*;
use samp_sdk::types::Cell;
use samp_sdk::amx::{AmxResult, AMX};

use eliza::Eliza;

define_native!(create,path_to_script:String);
define_native!(response,bot:usize,query:String,string:ref Cell,size:usize);

pub struct ChatBot{
    bots:Vec<Eliza>,
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
            Ok(_) => log!("[ChatBotPlugin] Natives are successful loaded"),
            Err(err) => log!("[ChatBotgitPlugin] Failed to register amx instance {:?}", err),
        }

        AMX_ERR_NONE
    }

    pub fn amx_unload(&self, _amx: &AMX) -> Cell {
        AMX_ERR_NONE
    }
    
    pub fn create(&mut self,_:&AMX,path_to_script:String) -> AmxResult<Cell>{
        let full_path = "scripts/".to_owned() + path_to_script.as_str();
        match Eliza::new(full_path.as_str()) {
            Ok(bot) => {
                self.bots.push(bot);
                Ok(self.bots.len() as Cell - 1)
            },
            Err(_) => Ok(-1),
        }
    }
    pub fn response(&mut self,_:&AMX,bot:usize,query:String, string:&mut Cell,size:usize) -> AmxResult<Cell>{
        let encoded = samp_sdk::cp1251::encode(self.bots[bot].respond(query.as_str()).as_str())?;
        set_string!(encoded,string,size);
        Ok(1)
    }

    
}

impl Default for ChatBot {
    fn default() -> Self {
        ChatBot{
            bots: Vec::new(),
        }
    }
}