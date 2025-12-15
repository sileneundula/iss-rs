use serde::{Serialize,Deserialize};
use std::fs::File;
use std::path::{PathBuf,Path};
use iss_rs::core::blobs::Piece;
use iss_rs::core::blobs::PIECE_SIZE_IN_BYTES;
use pretty_env_logger;
use dotenvy;
use log;
use std::boxed::Box;

/// TODO:
/// 
///     - [ ] Hash File Extension on top of the file (so that there are no mismatches in file extensions)
///     - [ ] Check by Magic Bytes
/// 
/// 
/// - [ ] File Parsing
///     - [X] Get File Pieces
///     - [ ] Rebuild Files
///     - [ ] Store as piece (maybe switch to just 256kb as range and keep files unpadded)
///     - [ ] Piece Length Parsing
///         - [ ] Number of Pieces
///         - [ ] 
///     - [ ] (Important) File Piecing
///         - [ ] Naive piece matching
///         - [ ] More Advanced Version Of Piece Matching

/// Represents a File
#[derive(Serialize,Deserialize,Debug,Clone,Hash,PartialEq,PartialOrd)]
pub struct LemurFileBytes {
    pub bytes: Vec<u8>,
}

#[derive(Serialize,Deserialize,Debug,Clone,Hash,PartialEq,PartialOrd)]
/// Represents a File Extension
pub struct LemurFileExtension {
    pub extension: String,
    pub magic_bytes: Option<[u8;4]>,
}

impl LemurFileBytes {
    pub fn new() {

    }
    /// # Read File From Path
    pub fn from_path<T: AsRef<Path>>(path: T) -> Self {
        let bytes = std::fs::read(path.as_ref()).expect("Failed To Read File");
        
        return Self {
            bytes: bytes
        }
    }
    /// Gets Length of Bytes
    pub fn get_length(&self) -> usize {
        self.bytes.len()
    }
    /// # Get Pieces For File
    /// 
    /// This gets the number of pieces and piece type for a given file (as of bytes).
    pub fn parse_file_length(&self) -> (Pieces, usize) {
        let length = self.get_length();

        if length <= 4096 {
            return (Pieces::TinyPiece,1usize)
        }
        else if length < 16_384 {
            let pieces: usize = usize::div_ceil(length, 16_384);
            return (Pieces::TinyPiece,pieces)
        }
        else if length == 16_384 {
            return (Pieces::MiniPiece,1)
        }
        else if length < 65_536 {
            let pieces = usize::div_ceil(length, 16_384);
            return (Pieces::MiniPiece,pieces)
        }
        else if length == 65_536 {
            return (Pieces::MiniPiece,4);
        }
        else if length < 262_144 {
            let pieces: usize = usize::div_ceil(length, 16_384);
            return (Pieces::MiniPiece, pieces)
        }
        else if length == 262_144 {
            return (Pieces::Piece,1);
        }
        else if length > 262_144 {
            // Ceil the value as usize
            let pieces: usize = usize::div_ceil(length, 262_144);
            return (Pieces::Piece,pieces)
        }
        else {
            panic!("Unknown Error")
        }
    }
    pub fn into_pieces(&self) -> Vec<Box<Piece>> {
        let parsed = self.parse_file_length();
        let mut pieces: Vec<Box<Piece>> = vec![];

        if parsed.0 == Pieces::Piece {
            let mut cursor: usize = 0usize;
            
            for i in 0..parsed.1-1 {
                let mut cursor_added = cursor + PIECE_SIZE_IN_BYTES;
                let y: &[u8] = &self.bytes[cursor..cursor_added];
                println!("Cursor Added: {}", cursor_added);
                let x: Piece = iss_rs::core::blobs::Piece::from_bytes(y);
                pieces.push(Box::new(x));
                cursor += PIECE_SIZE_IN_BYTES;
            }
            println!("Cursor: {}", cursor);
            let y = &self.bytes[cursor..self.get_length()];
            let x = Box::new(Piece::new(y));
            pieces.push(x);
        }
        return pieces
    }


}

#[derive(Serialize,Deserialize,Debug,Clone,Hash,PartialEq,PartialOrd)]
pub enum Pieces {
    TinyPiece, // 4kb
    MiniPiece, //16kb
    Piece, // 256kb
}

pub mod encoder;

#[test]
fn read_file() {
    println!("Reading File...");
    let file = LemurFileBytes::from_path("C:\\Users\\silen\\dexandlexy\\4893b1bcd74b99ca32380ba83fad8fee.png");
    println!("Getting File Length...");
    let length = file.get_length();
    println!("File Length: {}", length);
    let parsed = file.parse_file_length();
    println!("File Piece Type: {:?}", parsed.0);
    println!("Number of Pieces: {}", parsed.1);
}

#[test]
fn read_file_larger() {
    pretty_env_logger::init();
    log::trace!("Test");
    
    println!("Reading File... 256kb");
    let file = LemurFileBytes::from_path("C:\\Users\\silen\\Downloads\\eset_smart_security_premium_live_installer.exe");
    println!("Getting File Length...");
    let length = file.get_length();
    println!("File Length: {}", length);
    let parsed = file.parse_file_length();
    println!("File Piece Type: {:?}", parsed.0);
    println!("Number of Pieces: {}", parsed.1);
    
    let pieces = file.into_pieces();
    
    for piece in pieces {
        println!("BLAKE3: {}",piece.digest());
    }
}