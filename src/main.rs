extern crate discord;
extern crate conrod;

use std::env;
use std::thread;
use std::io;
use std::sync::mpsc;
use discord::Discord;
use discord::model::Event;
use discord::model::ChannelId;

fn chat_action(chat_info_rv : mpsc::Receiver<Box<String>>) {
    loop {
	        if let Ok(msg) = chat_info_rv.recv() {
                println!("{}", msg);
            }
        }
}

fn read_chat(mut connection : &mut discord::Connection, chat_info_tr : mpsc::Sender<Box<String>>) {
    loop {
    	match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => 
            {
                println!("{} says: {}, channel id: {}", message.author.name, message.content, message.channel_id);

    		    if message.content == "!test" {
                     chat_info_tr.send(Box::new(message.content)).unwrap();
                }
    	    }
    	    Ok(_) => {}
    	    Err(discord::Error::Closed(code, body)) => {
			    println!("Gateway closed on us with code {:?}: {}", code, body);	
    	    }
		    Err(err) => println!("Recieve error: {:?}", err)
        }
    }
}

fn read_console (msg_content_tr : mpsc::Sender<Box<String>>){
    loop {
    	let mut msg = String::new();
        println!("Enter the number of the channel and then the msg");
    	loop {
    		io::stdin().read_line(&mut msg).unwrap();
            msg.pop();
    		if msg == "1" || msg == "2" {
    			io::stdin().read_line(&mut msg).unwrap();
    			msg_content_tr.send(Box::new(msg.clone())).unwrap();
                println!("Sent!");
                msg.clear();
            }
            else {
                println!("Mate, it doesn't work that way.\n 1) Channel number \n 2) Message");
    		}
    		msg.clear();
    	}
    }
}

fn message_send (discord : discord::Discord, msg_content_rv : mpsc::Receiver<Box<String>>) {
	let channel_id_2 = ChannelId(242785065422159872);
    let channel_id_1 = ChannelId(242782392362860546);
    loop {
        if let Ok(mut msg) = msg_content_rv.recv() {
    	    match msg.remove(0) {
			    '1' => {let _ = discord.send_message(&channel_id_1, &msg, "", false);}
   			    '2' => {let _ = discord.send_message(&channel_id_2, &msg, "", false);}
                _ => {println!("oopsie!");}
    	    }
        }
    }
}


fn main() {
	let login = env::var("LOGIN").unwrap();
	let password = env::var("PASSWORD").unwrap();

//    let login = gui_r.recv().unwrap();
//  let password = gui_r.recv().unwrap();
	let discord = Discord::new(&login, &password).expect("login failed");

	let (mut connection, _readyevent) = discord.connect().expect("connect failed");

    let (chat_info_tr, chat_info_rv) = mpsc::channel();
    let (msg_content_tr, msg_content_rv) = mpsc::channel();
    println!("Ready.");
    thread::spawn(move || read_chat(&mut connection, chat_info_tr));

    thread::spawn(move || read_console(msg_content_tr));
    thread::spawn(move || chat_action(chat_info_rv));
    message_send(discord, msg_content_rv);
}

