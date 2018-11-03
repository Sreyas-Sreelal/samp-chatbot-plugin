#include<a_samp>
#include<zcmd>
#include<chatbot>

new ChatBot:bot;
public OnFilterScriptInit(){
	bot = ChatBotCreate("sample.json");
	if(bot == CHAT_BOT_ERROR){
		printf("Failed to create a bot instance!!");
	}
	
	return 1;
}
CMD:say(playerid,params[]){
	new response[128];
	BotResponse(bot,params,response);
	format(response,sizeof(response),"Bot : %s",response);
	SendClientMessage(playerid, -1,response);
	return 1;
}	