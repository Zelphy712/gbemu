use std::fs;
use std::str;
mod tables;

#[derive(Debug)]
struct GbHeader {
    title: String,
    rom_size: u8,
    ram_size: u8,
    cartridge_type: u8,
    destination_code: u8,
    header_checksum: u8,
    logo: Vec<u8>,
}

impl GbHeader {
    fn from_bytes(bytes: &[u8]) -> Self {
        let game_title: String = match str::from_utf8(&bytes[0x0134..0x0144]) {
            Ok(title) => title.trim_end_matches('\0').to_string(),
            Err(_) => "Invalid ASCII data".to_string(),
        };

        Self {
            title: game_title,
            cartridge_type: bytes[0x0147],
            rom_size: bytes[0x0148],
            ram_size: bytes[0x0149],
            header_checksum: bytes[0x014D],
            destination_code: bytes[0x014A],
            logo: bytes[0x0104..0x0134].to_vec(),
        }
    }

    fn check_logo(&self) -> bool {
        for i in 0..=47 {
            if self.logo[i] != tables::LOGO_DATA[i] {
                return false;
            }
        }
        true
    }
}

fn main() {
    // let mut file: fs::File = fs::File::open("../../roms/tetris.gb")?;

    let file_contents: Vec<u8> =
        fs::read("/home/el2316/dev/git/gbemu/roms/tetris.gb").expect("Expecting a file");

    let header: GbHeader = GbHeader::from_bytes(&file_contents);

    if header.check_logo() {
        println!("logo is good!")
    } else {
        println!("logo is bad!")
    }

    // println!("File contents:\n{:?}", file_contents);
    println!("Header: {:?}", header);

    println!("ram size: {}", tables::get_ram_size(4).unwrap_or(0));
}
