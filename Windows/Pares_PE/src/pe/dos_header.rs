use std::convert::TryInto;

#[repr(C)]
#[derive(Debug)]
pub struct DosHeader {
    pub e_magic: u16,
    pub e_cblp: u16,
    pub e_cp: u16,
    pub e_crlc: u16,
    pub e_cparhdr: u16,
    pub e_minalloc: u16,
    pub e_maxalloc: u16,
    pub e_ss: u16,
    pub e_sp: u16,
    pub e_csum: u16,
    pub e_ip: u16,
    pub e_cs: u16,
    pub e_lfarlc: u16,
    pub e_ovno: u16,
    pub e_res: [u16; 4],
    pub e_oemid: u16,
    pub e_oeminfo: u16,
    pub e_res2: [u16; 10],
    pub e_lfanew: u32,
}


pub fn parse_dos_header(data: &[u8]) -> DosHeader {
    DosHeader {
        e_magic:   u16::from_le_bytes(data[0..2].try_into().unwrap()),
        e_cblp:    u16::from_le_bytes(data[2..4].try_into().unwrap()),
        e_cp:      u16::from_le_bytes(data[4..6].try_into().unwrap()),
        e_crlc:    u16::from_le_bytes(data[6..8].try_into().unwrap()),
        e_cparhdr: u16::from_le_bytes(data[8..10].try_into().unwrap()),
        e_minalloc:u16::from_le_bytes(data[10..12].try_into().unwrap()),
        e_maxalloc:u16::from_le_bytes(data[12..14].try_into().unwrap()),
        e_ss:      u16::from_le_bytes(data[14..16].try_into().unwrap()),
        e_sp:      u16::from_le_bytes(data[16..18].try_into().unwrap()),
        e_csum:    u16::from_le_bytes(data[18..20].try_into().unwrap()),
        e_ip:      u16::from_le_bytes(data[20..22].try_into().unwrap()),
        e_cs:      u16::from_le_bytes(data[22..24].try_into().unwrap()),
        e_lfarlc:  u16::from_le_bytes(data[24..26].try_into().unwrap()),
        e_ovno:    u16::from_le_bytes(data[26..28].try_into().unwrap()),
        e_res: [
            u16::from_le_bytes(data[28..30].try_into().unwrap()),
            u16::from_le_bytes(data[30..32].try_into().unwrap()),
            u16::from_le_bytes(data[32..34].try_into().unwrap()),
            u16::from_le_bytes(data[34..36].try_into().unwrap()),
        ],
        e_oemid:   u16::from_le_bytes(data[36..38].try_into().unwrap()),
        e_oeminfo: u16::from_le_bytes(data[38..40].try_into().unwrap()),
        e_res2: [
            u16::from_le_bytes(data[40..42].try_into().unwrap()),
            u16::from_le_bytes(data[42..44].try_into().unwrap()),
            u16::from_le_bytes(data[44..46].try_into().unwrap()),
            u16::from_le_bytes(data[46..48].try_into().unwrap()),
            u16::from_le_bytes(data[48..50].try_into().unwrap()),
            u16::from_le_bytes(data[50..52].try_into().unwrap()),
            u16::from_le_bytes(data[52..54].try_into().unwrap()),
            u16::from_le_bytes(data[54..56].try_into().unwrap()),
            u16::from_le_bytes(data[56..58].try_into().unwrap()),
            u16::from_le_bytes(data[58..60].try_into().unwrap()),
        ],
        e_lfanew:  u32::from_le_bytes(data[60..64].try_into().unwrap()),
    }
}
