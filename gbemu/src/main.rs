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

#[derive(Debug)]
enum GbHeaderError {
    InvalidTitle,
    NotEnoughData,
}

impl GbHeader {
    fn from_bytes(bytes: &[u8]) -> Result<Self, GbHeaderError> {
        if bytes.len() < 0x014F + 1 {
            return Err(GbHeaderError::NotEnoughData);
        }

        let game_title = std::str::from_utf8(&bytes[0x0134..0x0144])
            .map_err(|_| GbHeaderError::InvalidTitle)?
            .trim_end_matches('\0')
            .to_string();

        Ok(Self {
            title: game_title,
            cartridge_type: bytes[0x0147],
            rom_size: bytes[0x0148],
            ram_size: bytes[0x0149],
            header_checksum: bytes[0x014D],
            destination_code: bytes[0x014A],
            logo: bytes[0x0104..0x0134].to_vec(),
        })
    }

    fn check_logo(&self) -> bool {
        self.logo
            .iter()
            .zip(tables::LOGO_DATA.iter())
            .all(|(a, b)| a == b)
    }
}

fn main() {
    // let mut file: fs::File = fs::File::open("../../roms/tetris.gb")?;

    let file_contents: Vec<u8> =
        std::fs::read("/home/el2316/dev/git/gbemu/roms/tetris.gb").expect("Expecting a file");

    let header: Result<GbHeader, GbHeaderError> = GbHeader::from_bytes(&file_contents);

    match header {
        Ok(header) => {
            if header.check_logo() {
                println!("logo is good!")
            } else {
                println!("logo is bad!")
            }
            println!("Header: {:?}", header);
        }
        Err(e) => {
            println!("Failed to parse header: {:?}", e);
        }
    }
    println!("ram size: {}", tables::get_ram_size(4).unwrap_or(0));
}
