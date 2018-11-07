/*==============================================================================
	
	Chatbot Plugin for SA:MP

		Copyright (C) 2018 Sreyas Sreelal
		This program is free software: you can redistribute it and/or modify it
		under the terms of the GNU General Public License as published by the
		Free Software Foundation, either version 3 of the License, or (at your
		option) any later version.
		This program is distributed in the hope that it will be useful, but
		WITHOUT ANY WARRANTY; without even the implied warranty of
		MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
		See the GNU General Public License for more details.
		You should have received a copy of the GNU General Public License along
		with this program.  If not, see <http://www.gnu.org/licenses/>.
==============================================================================*/

#define CHAT_BOT_ERROR (ChatBot:-1)


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

native ChatBot:ChatBotCreate(path[]);

/*
	Native: BotResponse
	
	Description:
		Generates a response from the query send to the bot
	
	Parameters:
		- `ChatBot:bot`: Bot instance id
		- `query[]`: Message to send to user
		- `string[]`: String to store response from bot
		- `size`: size of the destination string
	
	Returns:
		- `1`: Success
		- `0`: Failed

*/
native BotResponse(ChatBot:bot,query[],string[],size = sizeof(string));

/*
	Native: BotGreet
	
	Description:
		Randomly selects a greeting statement from the greetings list in the script.
	
	Parameters:
		- `ChatBot:bot`: Bot instance id
		- `message[]`: String to store response from bot

	Returns:
		- `1`: Success
		- `0`: Failed

*/
native BotGreet(ChatBot:bot,message[],size=sizeof(message));

/*
	Native: BotFarewell
	
	Description:
		Randomly selects a farewell statement from the farewell list in the script.
	
	Parameters:
		- `ChatBot:bot`: Bot instance id
		- `message[]`: String to store response from bot
	
	Returns:
		- `1`: Success
		- `0`: Failed

*/
native BotFarewell(Chatbot:bot,message[],size=sizeof(message));



