/// # Storage Size (Config)
/// Unsigned4: 0-F
/// Unsigned8: 00-FF
/// Unsigned16: 0000-FFFF
pub enum StorageSize {
    Unsigned4, // Nibble, 4 bits | 0-F
    Unsigned8, // Byte, 8 bits (1 byte)
    Unsigned16 // Large, 16 bits (2 bytes)
}