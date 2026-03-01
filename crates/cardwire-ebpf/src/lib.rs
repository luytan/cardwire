use aya::maps::{HashMap, MapError};
use aya::programs::Lsm;
use aya::{Btf, Ebpf};
use std::io::{Error as IoError, ErrorKind};

pub struct EbpfBlocker {
    ebpf: Ebpf,
}

impl EbpfBlocker {
    fn missing_entity(kind: &str, name: &str) -> IoError {
        IoError::new(ErrorKind::NotFound, format!("{} not found: {}", kind, name))
    }

    fn pci_key(pci: &str) -> [u8; 16] {
        let mut key = [0u8; 16];
        let bytes = pci.as_bytes();
        let len = bytes.len().min(15);
        key[..len].copy_from_slice(&bytes[..len]);
        key[len] = 0;
        key
    }

    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut ebpf = Ebpf::load(aya::include_bytes_aligned!(concat!(
            env!("OUT_DIR"),
            "/bpf.o"
        )))?;

        let btf = Btf::from_sys_fs()?;
        let program: &mut Lsm = ebpf
            .program_mut("file_open")
            .ok_or_else(|| Self::missing_entity("program", "file_open"))?
            .try_into()?;
        program.load("file_open", &btf)?;
        program.attach()?;

        Ok(Self { ebpf })
    }

    pub fn block_id(&mut self, id: u32) -> Result<(), Box<dyn std::error::Error>> {
        let mut map: HashMap<_, u32, u8> = HashMap::try_from(
            self.ebpf
                .map_mut("BLOCKED_IDS")
                .ok_or_else(|| Self::missing_entity("map", "BLOCKED_IDS"))?,
        )?;
        map.insert(id, 1, 0)?;
        Ok(())
    }

    pub fn unblock_id(&mut self, id: u32) -> Result<(), Box<dyn std::error::Error>> {
        let mut map: HashMap<_, u32, u8> = HashMap::try_from(
            self.ebpf
                .map_mut("BLOCKED_IDS")
                .ok_or_else(|| Self::missing_entity("map", "BLOCKED_IDS"))?,
        )?;
        let _ = map.remove(&id);
        Ok(())
    }

    pub fn block_pci(&mut self, pci: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut map: HashMap<_, [u8; 16], u8> = HashMap::try_from(
            self.ebpf
                .map_mut("BLOCKED_PCI")
                .ok_or_else(|| Self::missing_entity("map", "BLOCKED_PCI"))?,
        )?;
        let key = Self::pci_key(pci);
        map.insert(key, 1, 0)?;
        Ok(())
    }

    pub fn unblock_pci(&mut self, pci: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut map: HashMap<_, [u8; 16], u8> = HashMap::try_from(
            self.ebpf
                .map_mut("BLOCKED_PCI")
                .ok_or_else(|| Self::missing_entity("map", "BLOCKED_PCI"))?,
        )?;
        let key = Self::pci_key(pci);
        let _ = map.remove(&key);
        Ok(())
    }

    pub fn is_id_blocked(&mut self, id: u32) -> Result<bool, Box<dyn std::error::Error>> {
        let map: HashMap<_, u32, u8> = HashMap::try_from(
            self.ebpf
                .map("BLOCKED_IDS")
                .ok_or_else(|| Self::missing_entity("map", "BLOCKED_IDS"))?,
        )?;
        match map.get(&id, 0) {
            Ok(_) => Ok(true),
            Err(MapError::KeyNotFound) => Ok(false),
            Err(err) => Err(err.into()),
        }
    }

    pub fn is_pci_blocked(&mut self, pci: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let map: HashMap<_, [u8; 16], u8> = HashMap::try_from(
            self.ebpf
                .map("BLOCKED_PCI")
                .ok_or_else(|| Self::missing_entity("map", "BLOCKED_PCI"))?,
        )?;
        let key = Self::pci_key(pci);
        match map.get(&key, 0) {
            Ok(_) => Ok(true),
            Err(MapError::KeyNotFound) => Ok(false),
            Err(err) => Err(err.into()),
        }
    }
}
