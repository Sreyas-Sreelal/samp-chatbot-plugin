#include<a_samp>
#include<zcmd>
#include<chatbot>

#define RED "{FF0000}"
#define WHITE "{FFFFFF}"

new ChatBot:bot;

public OnFilterScriptInit(){
	bot = ChatBotCreate("sample.json");
	if(bot == CHAT_BOT_ERROR){
		printf("Failed to create a bot instance!!");
	}

	return 1;
}

CMD:say(playerid,params[]){
	new name[MAX_PLAYER_NAME],msg[128],response[128];
    GetPlayerName(playerid, name,sizeof(name));
    format(msg, sizeof(msg),"%s: %s",name,params);
    SendClientMessageToAll(-1,msg);

	if(BotResponse(bot,params,response)){
		format(response,sizeof(response),RED "Bot:" WHITE " %s",response);
		SendClientMessage(playerid, -1,response);
	} else{
		SendClientMessage(playerid, -1,"Failed to fetch a response");
	}

	return 1;
}	