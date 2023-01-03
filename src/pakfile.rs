use std::io;

use super::Version;

pub struct PakFile {
    pub version: Version,
    pub footer: super::Footer,
    pub entries: hashbrown::HashMap<String, super::PakEntry>,
}

impl PakFile {
    pub fn new<R: io::Read + io::Seek>(
        version: super::Version,
        mut reader: R,
    ) -> Result<Self, super::Error> {
        reader.seek(io::SeekFrom::End(-(footer_size(&version) as i64)))?;
        // parse footer info to get index offset
        let footer = super::Footer::new(&mut reader, &version)?;
        reader.seek(io::SeekFrom::Start(footer.offset))?;
        Ok(Self {
            version,
            footer,
            entries: hashbrown::HashMap::new(),
        })
    }
}

fn footer_size(version: &Version) -> u32 {
    // (magic + version): u32 + (offset + size): u64 + hash: [u8; 20]
    let mut size = 4 * 2 + 8 * 2 + 20;
    if version >= &Version::IndexEncryption {
        // encrypted: bool
        size += 1;
    }
    if version >= &Version::EncryptionKeyGuid {
        // encryption guid: [u8; 20]
        size += 10;
    }
    if version >= &Version::FNameBasedCompression {
        // compression names: [[u8; 32]; 4]
        size += 32 * 4;
    }
    if version >= &Version::FrozenIndex {
        // extra compression name: [u8; 32]
        size += 32
    }
    if version == &Version::FrozenIndex {
        // frozen index: bool
        size += 1;
    }
    size
}