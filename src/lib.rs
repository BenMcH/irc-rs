use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;
use std::thread;

pub struct IrcConnection {
    connection: TcpStream,
    host      : String,
    channels  : Vec<String>,
    output    : Vec<String>,
}

impl IrcConnection {
    pub fn from_str(addr: &str, nick: &str, username: &str, name: &str) -> Self {
        let mut connection = IrcConnection {
            connection: TcpStream::connect(&addr[..]).expect("Failed to connect!"),
            host      : addr.to_owned(),
            channels  : vec![],
            output    : vec![]
        };
        connection.join(nick, username, name);
        let mut reader_conn = connection.connection.try_clone().unwrap();
        thread::spawn(move || {
            let reader = BufReader::new(reader_conn.try_clone().unwrap());
            for line in reader.lines() {
                let l = line.unwrap();
                println!("{}", l);
                if l.starts_with("PING") {
                    let (_, server) = l.split_at(4);
                    let pong = format!("PONG{}", server);
                    println!("{}", pong);
                    let _ = reader_conn.write(pong.as_bytes());
                }
            }
        });
        connection
    }

    pub fn get_connection(&mut self) -> &mut TcpStream {
        &mut self.connection
    }

    pub fn get_host(&self) -> &String {
        &self.host
    }

    pub fn join_channel(&mut self, channel: &str) {
        self.send_command(format!("JOIN {}\r\n", channel));
        self.channels.push(String::from(channel));
    }

    pub fn get_channels(self) -> Vec<String> {
        self.channels
    }

    pub fn message(&mut self, to: &str, message: &str) {
        self.send_command(format!("PRIVMSG {} :{}\r\n", to, message));
    }

    fn join(&mut self, nick: &str, username: &str, name: &str) {
        self.send_command(format!("NICK {}\r\nUSER {} 0 * :{}\r\n", nick, username, name));
    }

    pub fn get_output(&mut self) -> Option<String> {
        self.output.pop()
    }

    pub fn quit(&mut self, reason: String) {
        self.send_command(format!("QUIT {}", reason));
    }

    fn send_command(&mut self, msg: String) {
        let _ = self.connection.write(msg.as_bytes());
    }

}
