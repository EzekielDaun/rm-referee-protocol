use deku::DekuUpdate;
use rm_referee_protocol::*;

fn main() {
    println!("Hello, world!");

    // Example payload: Game Status (0x0001)
    let game_state = RefereeFrameCmdData::GameStatus {
        game_type: GameType::RMUA,
        game_progress: GameProgress::InProgress,
        stage_remain_time: 123,
        sync_time_stamp: 456,
    };
    println!("{:?}", game_state);
    let game_state_bytes: Vec<_> = game_state.clone().try_into().unwrap();
    println!("{:02X?}", game_state_bytes);
    let parsed: RefereeFrameCmdData = RefereeFrameCmdData::try_from(&game_state_bytes[..]).unwrap();
    println!("{:?}", parsed);
    assert_eq!(game_state, parsed);

    // Example frame header
    let mut header = RefereeFrameHeader {
        // header.data_length is payload length (excludes the 2-byte cmd_id)
        data_length: (game_state_bytes.len() - std::mem::size_of::<u16>()) as u16,
        seq: 0,
        crc8: 0,
    };
    header.update().unwrap();
    println!("{:?}", header);
    let header_bytes: Vec<_> = header.try_into().unwrap();
    println!("{:02X?}", header_bytes);
    let parsed_header: RefereeFrameHeader =
        RefereeFrameHeader::try_from(&header_bytes[..]).unwrap();
    println!("{:?}", parsed_header);
    assert_eq!(header, parsed_header);

    // Example full frame
    let mut frame = RefereeFrame {
        header,
        cmd_data: game_state,
        frame_tail: 0x0,
    };
    frame.update().unwrap();
    println!("{:?}", frame);
    let frame_bytes: Vec<_> = frame.clone().try_into().unwrap();
    println!("{:02X?}", frame_bytes);
    let parsed_frame: RefereeFrame = RefereeFrame::try_from(&frame_bytes[..]).unwrap();
    println!("{:?}", parsed_frame);
    assert_eq!(frame, parsed_frame);

    // Example of bad full frame CRC detection
    let bad_crc_frame_bytes: Vec<_> = {
        let mut v = frame_bytes.clone();
        let len = v.len();
        v[len - 1] ^= 0xFF;
        v
    };
    let parsed_bad_crc_frame = RefereeFrame::try_from(&bad_crc_frame_bytes[..]);
    println!("{:?}", parsed_bad_crc_frame);
    assert!(parsed_bad_crc_frame.is_err());

    // Example of bad header CRC detection
    let bad_crc_header_bytes: Vec<_> = {
        let mut v = header_bytes.clone();
        let len = v.len();
        v[len - 1] ^= 0xFF;
        v
    };
    let parsed_bad_crc_header = RefereeFrameHeader::try_from(&bad_crc_header_bytes[..]);
    println!("{:?}", parsed_bad_crc_header);
    assert!(parsed_bad_crc_header.is_err());
}
