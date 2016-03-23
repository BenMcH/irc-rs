extern crate irc;
use irc::IrcConnection;
use std::time::Duration;

fn main() {
    let mut conn: IrcConnection = IrcConnection::from_str("chat.freenode.net:6667", "test_nick", "test_username", "test");
    conn.join_channel("#dailyprogrammer");
    conn.message("#dailyprogrammer", "This is a test");
    std::thread::sleep(Duration::new(60, 0));
}
