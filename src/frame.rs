use crc::{Algorithm, Crc};
use deku::{DekuContainerWrite, deku_derive};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::RefereeFrameCmdData;

// CRC configurations
pub const RM_CRC8: Crc<u8> = Crc::<u8>::new(&Algorithm {
    width: 8,
    poly: 0x31,
    init: 0xFF,
    refin: false,
    refout: false,
    xorout: 0x00,
    check: 0xF7,
    residue: 0x00,
});

pub const RM_CRC16: Crc<u16> = Crc::<u16>::new(&Algorithm {
    width: 16,
    poly: 0x1021,
    init: 0xFFFF,
    refin: false,
    refout: false,
    xorout: 0x0000,
    check: 0x29B1,
    residue: 0x0000,
});

pub const FRAME_SOF: u8 = 0xA5;

#[deku_derive(DekuRead, DekuWrite)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[deku(magic = b"\xA5", endian = "little")]
pub struct RefereeFrameHeader {
    pub data_length: u16,
    pub seq: u8,
    #[deku(
        assert_eq = "RM_CRC8.checksum(&[FRAME_SOF,data_length.to_le_bytes()[0],data_length.to_le_bytes()[1],*seq])",
        update = "RM_CRC8.checksum(&[FRAME_SOF,self.data_length.to_le_bytes()[0],self.data_length.to_le_bytes()[1],self.seq])"
    )]
    pub crc8: u8,
}

#[deku_derive(DekuRead, DekuWrite)]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RefereeFrame {
    #[deku(update = "Self::recompute_header_for_frame(self)")]
    pub header: RefereeFrameHeader,
    #[deku(ctx = "Some(header)")]
    pub cmd_data: RefereeFrameCmdData,
    #[deku(
        assert_eq = "Self::referee_frame_crc16(&header, &cmd_data)",
        update = "Self::referee_frame_crc16(&self.header, &self.cmd_data)"
    )]
    pub frame_tail: u16,
}

impl RefereeFrame {
    fn recompute_header_for_frame(&self) -> RefereeFrameHeader {
        let cmd_bytes = self.cmd_data.to_bytes().unwrap();
        let data_length = (cmd_bytes.len() as u16).saturating_sub(2);
        let seq = self.header.seq;
        let crc8 = RM_CRC8.checksum(&[
            FRAME_SOF,
            data_length.to_le_bytes()[0],
            data_length.to_le_bytes()[1],
            seq,
        ]);
        RefereeFrameHeader {
            data_length,
            seq,
            crc8,
        }
    }

    fn referee_frame_crc16(header: &RefereeFrameHeader, cmd_data: &RefereeFrameCmdData) -> u16 {
        let mut v = header.to_bytes().unwrap();
        v.extend_from_slice(cmd_data.to_bytes().unwrap().as_slice());
        RM_CRC16.checksum(&v)
    }
}
