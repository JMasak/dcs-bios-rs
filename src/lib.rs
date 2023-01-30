use std::net::Ipv4Addr;
use socket2::Socket;

//see https://bluejekyll.github.io/blog/posts/multicasting-in-rust/
//see https://github.com/DCSFlightpanels/dcs-bios/blob/master/Scripts/DCS-BIOS/doc/developerguide.adoc

pub const OUTPUT_PORT: u16 = 5010;
pub const INPUT_PORT: u16 = 7778;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
