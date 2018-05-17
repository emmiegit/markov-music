/*
 * socket.rs
 *
 * markov-music - A music player that uses Markov chains to choose songs
 * Copyright (c) 2017-2018 Ammon Smith
 *
 * markov-music is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 2 of the License, or
 * (at your option) any later version.
 *
 * markov-music is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with markov-music.  If not, see <http://www.gnu.org/licenses/>.
 */

use Result;
use config::DaemonConfig;
use std::os::unix::net::UnixDatagram;
use std::process::exit;
use std::str;
use utils::empty_mut_str;

fn split_cmd(command: &mut str) -> (&mut str, &mut str) {
    match command.find(' ') {
        Some(idx) => command.split_at_mut(idx),
        None => (command, empty_mut_str()),
    }
}

#[derive(Debug)]
pub struct SocketServer {
    socket: UnixDatagram,
}

impl SocketServer {
    pub fn bind(config: &DaemonConfig) -> Result<Self> {
        let socket = UnixDatagram::bind(&config.socket)?;

        Ok(SocketServer { socket })
    }

    pub fn wait(&self) -> Result<()> {
        let mut buffer = [0; 4096];
        let (len, addr) = self.socket.recv_from(&mut buffer[..])?;
        let slice = &mut buffer[..len];
        let command = str::from_utf8_mut(slice)?;
        let response = self.respond(command)?;
        self.socket.send_to(response.as_bytes(), addr.as_pathname().unwrap())?;

        Ok(())
    }

    fn respond(&self, command: &mut str) -> Result<&'static str> {
        let (call, arg) = split_cmd(command);
        call.make_ascii_uppercase();

        let response = match call as &str {
            "PING" => "PONG",
            "RECOMMEND" => self.set_recommend(arg),
            "QUIT" => exit(0),
            _ => "NOCMD",
        };

        Ok(response)
    }

    fn set_recommend(&self, arg: &mut str) -> &'static str {
        arg.make_ascii_uppercase();

        let setting = match arg as &str {
            "ON" => Some(true),
            "OFF" => Some(false),
            _ => None,
        };

        match setting {
            Some(_b) => {
                println!("TODO set daemon enabled: {}", _b);
                "RECOMMEND"
            },
            None => "INVARG",
        }
    }
}
