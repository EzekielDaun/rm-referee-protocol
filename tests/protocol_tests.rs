use deku::{DekuContainerWrite, DekuUpdate};
use rm_referee_protocol::*;
use serde::{Serialize, de::DeserializeOwned};

fn json_roundtrip<T>(value: &T) -> T
where
    T: Serialize + DeserializeOwned,
{
    let s = serde_json::to_string(value).unwrap();
    serde_json::from_str::<T>(&s).unwrap()
}

fn build_game_status() -> RefereeFrameCmdData {
    RefereeFrameCmdData::GameStatus {
        game_type: GameType::RMUA,
        game_progress: GameProgress::InProgress,
        stage_remain_time: 123,
        sync_time_stamp: 456,
    }
}

#[test]
fn test_cmd_id_is_little_endian() {
    let data = build_game_status();
    let bytes = data.to_bytes().unwrap();
    assert_eq!(bytes[0], 0x01);
    assert_eq!(bytes[1], 0x00);
}

#[test]
fn test_header_crc8_includes_magic_and_is_correct() {
    let cmd = build_game_status();
    let cmd_bytes = cmd.to_bytes().unwrap();
    let mut header = RefereeFrameHeader {
        data_length: (cmd_bytes.len() - 2) as u16,
        ..Default::default()
    };
    header.update().unwrap();
    let expected = RM_CRC8.checksum(&[
        FRAME_SOF,
        header.data_length.to_le_bytes()[0],
        header.data_length.to_le_bytes()[1],
        header.seq,
    ]);
    assert_eq!(header.crc8, expected);
}

#[test]
fn test_frame_crc16_and_tail_endianness() {
    let cmd = build_game_status();
    let cmd_bytes = cmd.to_bytes().unwrap();
    let mut header = RefereeFrameHeader {
        data_length: (cmd_bytes.len() - 2) as u16,
        ..Default::default()
    };
    header.update().unwrap();

    let mut frame = RefereeFrame {
        header,
        cmd_data: cmd,
        frame_tail: 0,
    };
    frame.update().unwrap();

    let hb = frame.header.to_bytes().unwrap();
    let db = frame.cmd_data.to_bytes().unwrap();
    let expected = RM_CRC16.checksum(&[hb.as_slice(), db.as_slice()].concat());
    assert_eq!(frame.frame_tail, expected);

    let fb = frame.to_bytes().unwrap();
    let tail = u16::from_le_bytes([fb[fb.len() - 2], fb[fb.len() - 1]]);
    assert_eq!(tail, expected);
}

#[test]
fn test_0204_buff_bits_and_serde() {
    let v = RefereeFrameCmdData::Buff {
        recovery_buff: 10,
        cooling_buff: 5,
        defense_buff: 50,
        vulnerability_buff: 30,
        attack_buff: 50,
        energy_ge_50: true,
        energy_ge_30: true,
        energy_ge_15: false,
        energy_ge_5: false,
        energy_ge_1: true,
    };
    let bytes = v.to_bytes().unwrap();
    let parsed = RefereeFrameCmdData::try_from(bytes.as_slice()).unwrap();
    assert_eq!(v, parsed);
    let de = json_roundtrip(&v);
    assert_eq!(v, de);
}

#[test]
fn test_0209_rfid_bits_and_serde() {
    let v = RefereeFrameCmdData::RFIDStatus {
        our_base_buff: true,
        our_center_highland_buff: true,
        enemy_center_highland_buff: false,
        our_trapezoid_highland_buff: true,
        enemy_trapezoid_highland_buff: false,
        our_ramp_near_front: true,
        our_ramp_near_back: false,
        enemy_ramp_near_front: true,
        enemy_ramp_near_back: false,
        our_cross_center_lower: true,
        our_cross_center_upper: false,
        enemy_cross_center_lower: true,
        enemy_cross_center_upper: false,
        our_cross_road_lower: true,
        our_cross_road_upper: false,
        enemy_cross_road_lower: true,
        enemy_cross_road_upper: false,
        our_fortress_buff: true,
        our_outpost_buff: true,
        our_supply_nonoverlap_or_rmul: false,
        our_supply_overlap: false,
        our_big_island_buff: true,
        enemy_big_island_buff: false,
        center_buff_rmul_only: false,
        enemy_fortress_buff: true,
    };
    let bytes = v.to_bytes().unwrap();
    let parsed = RefereeFrameCmdData::try_from(bytes.as_slice()).unwrap();
    assert_eq!(v, parsed);
    let de = json_roundtrip(&v);
    assert_eq!(v, de);
}

#[test]
fn test_020c_radar_mark_bits_and_serde() {
    let v = RefereeFrameCmdData::RadarMarkData {
        enemy_hero_vulnerable: true,
        enemy_engineer_vulnerable: false,
        enemy_infantry3_vulnerable: true,
        enemy_infantry4_vulnerable: false,
        enemy_sentry_vulnerable: true,
    };
    let bytes = v.to_bytes().unwrap();
    let parsed = RefereeFrameCmdData::try_from(bytes.as_slice()).unwrap();
    assert_eq!(v, parsed);
    let de = json_roundtrip(&v);
    assert_eq!(v, de);
}

#[test]
fn test_0307_map_path_and_serde() {
    let delta_x: Vec<i8> = (0..49).map(|i| i as i8 - 24).collect();
    let delta_y: Vec<i8> = (0..49).map(|i| 24 - i as i8).collect();
    let v = RefereeFrameCmdData::MapPathData {
        intention: 1,
        start_position_x: 123,
        start_position_y: 456,
        delta_x,
        delta_y,
        sender_id: EndpointId::RedHeroClient,
    };
    let bytes = v.to_bytes().unwrap();
    assert_eq!(bytes.len(), 2 + 105);
    let parsed = RefereeFrameCmdData::try_from(bytes.as_slice()).unwrap();
    assert_eq!(v, parsed);
    let de = json_roundtrip(&v);
    assert_eq!(v, de);
}

#[test]
fn test_0302_custom_robot_data_json_and_bytes() {
    let v = RefereeFrameCmdData::CustomRobotData { data: [0xAB; 30] };
    let bytes = v.to_bytes().unwrap();
    assert_eq!(bytes.len(), 2 + 30);
    let parsed = RefereeFrameCmdData::try_from(bytes.as_slice()).unwrap();
    assert_eq!(v, parsed);
    let de = json_roundtrip(&v);
    assert_eq!(v, de);
}

#[test]
fn test_0303_map_command_json_and_bytes() {
    let v = RefereeFrameCmdData::MapCommand {
        target_position_x: 1.23,
        target_position_y: 4.56,
        cmd_keyboard: 0x11,
        target_robot_id: RobotID::RedHero,
        cmd_source: EndpointId::RedHeroClient,
    };
    let bytes = v.to_bytes().unwrap();
    // fields sum to 12 bytes (4+4+1+1+2)
    assert_eq!(bytes.len(), 2 + 12);
    let parsed = RefereeFrameCmdData::try_from(bytes.as_slice()).unwrap();
    assert_eq!(v, parsed);
    let de = json_roundtrip(&v);
    assert_eq!(v, de);
}

#[test]
fn test_0304_remote_control_json_and_bytes() {
    let v = RefereeFrameCmdData::RemoteControl {
        mouse_x: -120,
        mouse_y: 250,
        mouse_z: -5,
        left_button_down: 1,
        right_button_down: 0,
        keyboard_value: 0b_0000_0001_0001_0001,
        reserved: 0,
    };
    let bytes = v.to_bytes().unwrap();
    assert_eq!(bytes.len(), 2 + 12);
    let parsed = RefereeFrameCmdData::try_from(bytes.as_slice()).unwrap();
    assert_eq!(v, parsed);
    let de = json_roundtrip(&v);
    assert_eq!(v, de);
}

#[test]
fn test_0305_map_robot_data_json_and_bytes() {
    let v = RefereeFrameCmdData::MapRobotData {
        hero_position_x: 100,
        hero_position_y: 200,
        engineer_position_x: 300,
        engineer_position_y: 400,
        infantry_3_position_x: 500,
        infantry_3_position_y: 600,
        infantry_4_position_x: 700,
        infantry_4_position_y: 800,
        infantry_5_position_x: 900,
        infantry_5_position_y: 1000,
        sentry_position_x: 1100,
        sentry_position_y: 1200,
    };
    let bytes = v.to_bytes().unwrap();
    assert_eq!(bytes.len(), 2 + 24);
    let parsed = RefereeFrameCmdData::try_from(bytes.as_slice()).unwrap();
    assert_eq!(v, parsed);
    let de = json_roundtrip(&v);
    assert_eq!(v, de);
}

#[test]
fn test_0306_simulate_control_json_and_bytes() {
    let v = RefereeFrameCmdData::SimulateControl {
        keyboard_value: 0x1234,
        mouse_x_pos: 1500,
        mouse_left_state: 1,
        mouse_y_pos: 800,
        mouse_right_state: 0,
        reserved: 0,
    };
    let bytes = v.to_bytes().unwrap();
    assert_eq!(bytes.len(), 2 + 8);
    let parsed = RefereeFrameCmdData::try_from(bytes.as_slice()).unwrap();
    assert_eq!(v, parsed);
    let de = json_roundtrip(&v);
    assert_eq!(v, de);
}

#[test]
fn test_0308_custom_info_json_and_bytes() {
    let v = RefereeFrameCmdData::CustomInfo {
        sender_id: EndpointId::RedHeroClient,
        receiver_id: EndpointId::BlueHeroClient,
        data: [0x31; 30],
    };
    let bytes = v.to_bytes().unwrap();
    assert_eq!(bytes.len(), 2 + 34);
    let parsed = RefereeFrameCmdData::try_from(bytes.as_slice()).unwrap();
    assert_eq!(v, parsed);
    let de = json_roundtrip(&v);
    assert_eq!(v, de);
}

#[test]
fn test_0309_robot_custom_data_json_and_bytes() {
    let v = RefereeFrameCmdData::RobotCustomData { data: [0xCD; 30] };
    let bytes = v.to_bytes().unwrap();
    assert_eq!(bytes.len(), 2 + 30);
    let parsed = RefereeFrameCmdData::try_from(bytes.as_slice()).unwrap();
    assert_eq!(v, parsed);
    let de = json_roundtrip(&v);
    assert_eq!(v, de);
}

#[test]
fn test_end_to_end_frame_json_and_bytes() {
    let cmd = RefereeFrameCmdData::CustomInfo {
        sender_id: EndpointId::RedHeroClient,
        receiver_id: EndpointId::RedHeroClient,
        data: [0x55; 30],
    };
    let cmd_bytes = cmd.to_bytes().unwrap();
    let expected_data_len = (cmd_bytes.len() - 2) as u16;
    let mut frame = RefereeFrame {
        header: RefereeFrameHeader {
            data_length: 0,
            seq: 7,
            crc8: 0,
        },
        cmd_data: cmd.clone(),
        frame_tail: 0,
    };
    frame.update().unwrap();
    assert_eq!(frame.header.data_length, expected_data_len);
    let fb = frame.to_bytes().unwrap();
    assert_eq!(fb.len(), 5 + cmd_bytes.len() + 2);
    let parsed = RefereeFrame::try_from(fb.as_slice()).unwrap();
    assert_eq!(frame, parsed);
    let de = json_roundtrip(&frame);
    assert_eq!(frame, de);
}
