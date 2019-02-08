use crate::util::Result;
use log::{debug, error, info};
use protos::users::UserPing;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread;
pub fn run(addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr)?;

    info!("Listening on: {}", addr);
    for stream in listener.incoming() {
        let mut socket = stream?;
        thread::spawn(move || {
            match handle_stream(&mut socket) {
                Ok(_) => (),
                Err(e) => error!("Error While Handling Packet! {}", e)
            }
            info!("Closing Connection for socket {:?} !", socket.peer_addr());
        });
    }
    Ok(())
}

fn handle_stream(stream: &mut TcpStream) -> Result<()> {
    let buffers: Vec<_> = stream.bytes().map(|b| b.unwrap()).collect();
    let buffers: Vec<_> = buffers
        .split(|p| p == &b'\0')
        .take_while(|p| !p.is_empty())
        .collect();
    for buffer in buffers {
        let packet_id = buffer.first().unwrap_or(&0);
        let packet: Vec<_> = buffer.iter().skip(1).cloned().collect();
        debug!("------------START-------------");
        debug!("Buffer: {:0x?}", buffer);
        debug!("packet_id: {}", packet_id);
        debug!("Packet: {:0x?}", packet);
        match packet_id {
            1 => {
                let ping = protobuf::parse_from_bytes::<UserPing>(&packet)?;
                debug!("Got Username: {}", ping.get_username());
                debug!("------------END-------------");
            }
            _ => {
                error!("Error ..");
            }
        }
    }
    Ok(())
}
