use std::io::{Read, Write};
use std::net::TcpStream; 
use crate::error::MgError;

pub struct MgConnection {
    stream: TcpStream,
    pub bolt_version: u32
}  

impl MgConnection {
    pub fn connect(addr: &str) -> Result<Self, MgError> {
        let mut stream = TcpStream::connect(addr)?;
        let versions = [1, 3, 0, 0]; // Try Bolt v1 and v3
        let bolt_version = handshake(&mut stream, &versions)?;
        Ok(Self { stream, bolt_version })
    }
}

fn handshake(stream: &mut TcpStream, versions: &[u32]) -> Result<u32, MgError> {
    let mut handshake = vec![0x60, 0x60, 0xB0, 0x17];
    for &v in versions.iter().take(4) {
        handshake.extend_from_slice(&v.to_be_bytes());
    }
    stream.write_all(&handshake)?;

    let mut response = [0u8; 4];
    stream.read_exact(&mut response)?;
    Ok(u32::from_be_bytes(response))
}



