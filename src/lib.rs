#![no_std]

extern crate alloc;
use deku::deku_derive;
use serde::{Deserialize, Serialize};

pub mod robot_interaction;
pub use crate::robot_interaction::{
    FigureColor, FigureOperateType, FigureType, InteractionFigure, InteractionLayerDelete,
    LayerDeleteType, RadarCmd, RobotInteractionUserData, SentryCmd,
};

mod frame;
pub use frame::{FRAME_SOF, RM_CRC8, RM_CRC16, RefereeFrame, RefereeFrameHeader};
mod types;

#[deku_derive(DekuRead, DekuWrite)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[deku(
    id_type = "u16",
    ctx = "header:Option<&RefereeFrameHeader>",
    ctx_default = "None"
)]
pub enum RefereeFrameCmdData {
    #[deku(id = 0x0001)]
    GameStatus {
        game_type: GameType,
        game_progress: GameProgress,
        stage_remain_time: u16,
        sync_time_stamp: u64,
    },
    #[deku(id = 0x0002)]
    GameResult(GameResult),
    #[deku(id = 0x0003)]
    GameRobotHP {
        red_1_robot_hp: u16,
        red_2_robot_hp: u16,
        red_3_robot_hp: u16,
        red_4_robot_hp: u16,
        #[deku(temp, temp_value = "u16::default()")]
        reserved: u16,
        red_7_robot_hp: u16,
        red_outpost_hp: u16,
        red_base_hp: u16,
        blue_1_robot_hp: u16,
        blue_2_robot_hp: u16,
        blue_3_robot_hp: u16,
        blue_4_robot_hp: u16,
        #[deku(temp, temp_value = "u16::default()")]
        reserved2: u16,
        blue_7_robot_hp: u16,
        blue_outpost_hp: u16,
        blue_base_hp: u16,
    },
    #[deku(id = 0x0101)]
    EventData {
        // bit0-2
        #[deku(bits = 1)]
        supply_no_overlap_occupied: bool,
        #[deku(bits = 1)]
        supply_overlap_occupied: bool,
        #[deku(bits = 1)]
        supply_occupied_rmul: bool,
        // bit3-4
        #[deku(bits = 1)]
        small_energy_mechanism_active: bool,
        #[deku(bits = 1)]
        large_energy_mechanism_active: bool,
        // bit5-6, bit7-8
        center_highland_occupy: OccupyType,
        trapezoid_highland_occupy: OccupyType,
        // bit9-17
        #[deku(bits = 9)]
        dart_last_hit_time_s: u16,
        // bit18-20
        dart_last_hit_target: DartTarget,
        // bit21-22, bit23-24
        center_buff_occupy: OccupyType,
        fortress_buff_occupy: OccupyType,
        // bit25-31 reserved
        #[deku(bits = 7, temp, temp_value = "u8::default()")]
        reserved_25_31: u8,
    },
    #[deku(id = 0x0104)]
    RefereeWarning {
        level: RefereeWarningLevel,
        offending_robot_id: RobotID,
        count: u8,
    },
    #[deku(id = 0x0105)]
    DartInfo {
        dart_remaining_time: u8,
        dart_last_hit_target: DartTarget,
        #[deku(bits = 3)]
        opponent_target_hit_count: u8,
        dart_selected_target: DartSelectionTarget,
        #[deku(bits = 8, temp, temp_value = "u8::default()")]
        reserved: u8,
    },
    #[deku(id = 0x0201)]
    RobotStatus {
        robot_id: RobotID,
        robot_level: u8,
        current_hp: u16,
        maximum_hp: u16,
        shooter_barrel_cooling_value: u16,
        shooter_barrel_heat_limit: u16,
        chassis_power_limit: u16,
        #[deku(bits = 1)]
        power_management_gimbal_output: bool,
        #[deku(bits = 1)]
        power_management_chassis_output: bool,
        #[deku(bits = 1)]
        power_management_shooter_output: bool,
        #[deku(bits = 5, temp, temp_value = "u8::default()")]
        reserved: u8,
    },
    #[deku(id = 0x0202)]
    PowerHeatData {
        #[deku(temp, temp_value = "u16::default()")]
        reserved1: u16,
        #[deku(temp, temp_value = "u16::default()")]
        reserved2: u16,
        #[deku(temp, temp_value = "f32::default()")]
        reserved3: f32,
        buffer_energy: u16,
        shooter_17mm_1_barrel_heat: u16,
        shooter_17mm_2_barrel_heat: u16,
        shooter_42mm_barrel_heat: u16,
    },
    #[deku(id = 0x0203)]
    RobotPos { x: f32, y: f32, angle: f32 },
    #[deku(id = 0x0204)]
    Buff {
        recovery_buff: u8,
        cooling_buff: u8,
        defense_buff: u8,
        vulnerability_buff: u8,
        attack_buff: u16,
        // remaining_energy (1 byte) split into bit flags per spec
        // bit0..bit4 indicate thresholds: ≥50%, ≥30%, ≥15%, ≥5%, ≥1%
        #[deku(bits = 1)]
        energy_ge_50: bool,
        #[deku(bits = 1)]
        energy_ge_30: bool,
        #[deku(bits = 1)]
        energy_ge_15: bool,
        #[deku(bits = 1)]
        energy_ge_5: bool,
        #[deku(bits = 1)]
        energy_ge_1: bool,
        #[deku(bits = 3, temp, temp_value = "u8::default()")]
        energy_reserved: u8,
    },
    #[deku(id = 0x0206)]
    HurtData {
        #[deku(bits = 4)]
        armor_id: u8,
        hp_deduction_reason: HpDeductionReason,
    },
    #[deku(id = 0x0207)]
    ShootData {
        bullet_type: BulletType,
        shooter_number: ShooterNumber,
        launching_frequency: u8,
        initial_speed: f32,
    },
    #[deku(id = 0x0208)]
    ProjectileAllowance {
        projectile_allowance_17mm: u16,
        projectile_allowance_42mm: u16,
        remaining_gold_coin: u16,
        projectile_allowance_fortress: u16,
    },
    #[deku(id = 0x0209)]
    RFIDStatus {
        // bit0..bit24: RFID points present; bit25..31 reserved
        #[deku(bits = 1)]
        our_base_buff: bool,
        #[deku(bits = 1)]
        our_center_highland_buff: bool,
        #[deku(bits = 1)]
        enemy_center_highland_buff: bool,
        #[deku(bits = 1)]
        our_trapezoid_highland_buff: bool,
        #[deku(bits = 1)]
        enemy_trapezoid_highland_buff: bool,
        #[deku(bits = 1)]
        our_ramp_near_front: bool,
        #[deku(bits = 1)]
        our_ramp_near_back: bool,
        #[deku(bits = 1)]
        enemy_ramp_near_front: bool,
        #[deku(bits = 1)]
        enemy_ramp_near_back: bool,
        #[deku(bits = 1)]
        our_cross_center_lower: bool,
        #[deku(bits = 1)]
        our_cross_center_upper: bool,
        #[deku(bits = 1)]
        enemy_cross_center_lower: bool,
        #[deku(bits = 1)]
        enemy_cross_center_upper: bool,
        #[deku(bits = 1)]
        our_cross_road_lower: bool,
        #[deku(bits = 1)]
        our_cross_road_upper: bool,
        #[deku(bits = 1)]
        enemy_cross_road_lower: bool,
        #[deku(bits = 1)]
        enemy_cross_road_upper: bool,
        #[deku(bits = 1)]
        our_fortress_buff: bool,
        #[deku(bits = 1)]
        our_outpost_buff: bool,
        #[deku(bits = 1)]
        our_supply_nonoverlap_or_rmul: bool,
        #[deku(bits = 1)]
        our_supply_overlap: bool,
        #[deku(bits = 1)]
        our_big_island_buff: bool,
        #[deku(bits = 1)]
        enemy_big_island_buff: bool,
        #[deku(bits = 1)]
        center_buff_rmul_only: bool,
        #[deku(bits = 1)]
        enemy_fortress_buff: bool,
        #[deku(bits = 7, temp, temp_value = "u8::default()")]
        rfid_reserved_25_31: u8,
    },
    #[deku(id = 0x020A)]
    DartClientCmd {
        dart_launch_opening_status: u8,
        #[deku(temp, temp_value = "u8::default()")]
        reserved: u8,
        target_change_time: u16,
        latest_launch_cmd_time: u16,
    },
    #[deku(id = 0x020B)]
    GroundRobotPosition {
        hero_x: f32,
        hero_y: f32,
        engineer_x: f32,
        engineer_y: f32,
        standard_3_x: f32,
        standard_3_y: f32,
        standard_4_x: f32,
        standard_4_y: f32,
        #[deku(temp, temp_value = "f32::default()")]
        reserved1: f32,
        #[deku(temp, temp_value = "f32::default()")]
        reserved2: f32,
    },
    #[deku(id = 0x020C)]
    RadarMarkData {
        // 对方各机器人易伤标记：达到 100 时置 1，否则 0
        #[deku(bits = 1)]
        enemy_hero_vulnerable: bool,
        #[deku(bits = 1)]
        enemy_engineer_vulnerable: bool,
        #[deku(bits = 1)]
        enemy_infantry3_vulnerable: bool,
        #[deku(bits = 1)]
        enemy_infantry4_vulnerable: bool,
        #[deku(bits = 1)]
        enemy_sentry_vulnerable: bool,
        #[deku(bits = 3, temp, temp_value = "u8::default()")]
        mark_reserved_5_7: u8,
    },
    #[deku(id = 0x020D)]
    SentryInfo {
        sentry_info: u32,
        sentry_info_2: u16,
    },
    #[deku(id = 0x020E)]
    RadarInfo {
        #[deku(bits = 2)]
        double_vulnerability_chance: u8,
        #[deku(bits = 1)]
        enemy_in_double_vulnerability: bool,
        #[deku(bits = 5, temp, temp_value = "u8::default()")]
        reserved: u8,
    },
    #[deku(id = 0x0301)]
    RobotInteractionData {
        data_cmd_id: u16,
        sender_id: EndpointId,
        receiver_id: EndpointId,
        #[deku(count = "header.unwrap().data_length as usize - 6")]
        user_data: alloc::vec::Vec<u8>,
    },
    // 图传链路：自定义控制器→机器人：表 4-1 0x0302（30 字节）
    #[deku(id = 0x0302)]
    CustomRobotData { data: [u8; 30] },
    // 选手端小地图交互数据（选手端→服务器→机器人）：表 3-1 0x0303
    #[deku(id = 0x0303)]
    MapCommand {
        target_position_x: f32,
        target_position_y: f32,
        cmd_keyboard: u8,
        target_robot_id: RobotID,
        cmd_source: EndpointId,
    },
    // 图传链路：键鼠遥控数据：表 4-3 0x0304（12 字节）
    #[deku(id = 0x0304)]
    RemoteControl {
        mouse_x: i16,
        mouse_y: i16,
        mouse_z: i16,
        left_button_down: u8,
        right_button_down: u8,
        keyboard_value: u16,
        reserved: u16,
    },
    // 选手端小地图接收雷达数据：表 3-2 0x0305（24 字节）
    #[deku(id = 0x0305)]
    MapRobotData {
        hero_position_x: u16,
        hero_position_y: u16,
        engineer_position_x: u16,
        engineer_position_y: u16,
        infantry_3_position_x: u16,
        infantry_3_position_y: u16,
        infantry_4_position_x: u16,
        infantry_4_position_y: u16,
        infantry_5_position_x: u16,
        infantry_5_position_y: u16,
        sentry_position_x: u16,
        sentry_position_y: u16,
    },
    // 非链路：自定义控制器模拟键鼠操作选手端：表 5-1 0x0306（8 字节）
    #[deku(id = 0x0306)]
    SimulateControl {
        keyboard_value: u16,
        #[deku(bits = 12)]
        mouse_x_pos: u16,
        #[deku(bits = 4)]
        mouse_left_state: u8,
        #[deku(bits = 12)]
        mouse_y_pos: u16,
        #[deku(bits = 4)]
        mouse_right_state: u8,
        reserved: u16,
    },
    // 选手端小地图接收路径数据：表 3-3 0x0307（103 字节）
    #[deku(id = 0x0307)]
    MapPathData {
        intention: u8,
        start_position_x: u16,
        start_position_y: u16,
        #[deku(count = "49")]
        delta_x: alloc::vec::Vec<i8>,
        #[deku(count = "49")]
        delta_y: alloc::vec::Vec<i8>,
        sender_id: EndpointId,
    },
    // 选手端小地图接收机器人消息：表 3-4 0x0308（34 字节）
    #[deku(id = 0x0308)]
    CustomInfo {
        sender_id: EndpointId,
        receiver_id: EndpointId,
        data: [u8; 30],
    },

    // 图传链路：机器人→自定义控制器：表 4-2 0x0309（30 字节）
    #[deku(id = 0x0309)]
    RobotCustomData { data: [u8; 30] },
}

impl RefereeFrameCmdData {
    // 尝试将 0x0301 的数据解析为 0x0120 哨兵指令
    pub fn as_sentry_cmd(&self) -> Option<SentryCmd> {
        if let RefereeFrameCmdData::RobotInteractionData {
            data_cmd_id,
            user_data,
            ..
        } = self
        {
            if *data_cmd_id == 0x0120 && user_data.len() == 4 {
                return SentryCmd::try_from(user_data.as_slice()).ok();
            }
        }
        None
    }

    // 尝试将 0x0301 的数据解析为 0x0121 雷达指令
    pub fn as_radar_cmd(&self) -> Option<RadarCmd> {
        if let RefereeFrameCmdData::RobotInteractionData {
            data_cmd_id,
            user_data,
            ..
        } = self
        {
            if *data_cmd_id == 0x0121 && user_data.len() == 1 {
                return RadarCmd::try_from(user_data.as_slice()).ok();
            }
        }
        None
    }

    // 将 0x0301 的数据解析为已知子内容枚举
    pub fn as_robot_interaction(&self) -> Option<RobotInteractionUserData> {
        if let RefereeFrameCmdData::RobotInteractionData {
            data_cmd_id,
            user_data,
            ..
        } = self
        {
            match *data_cmd_id {
                0x0100 => InteractionLayerDelete::try_from(user_data.as_slice())
                    .ok()
                    .map(RobotInteractionUserData::LayerDelete),
                0x0101 => InteractionFigure::try_from(user_data.as_slice())
                    .ok()
                    .map(RobotInteractionUserData::Figure),
                0x0102 => {
                    if user_data.len() == 30 {
                        let a = InteractionFigure::try_from(&user_data[0..15]).ok()?;
                        let b = InteractionFigure::try_from(&user_data[15..30]).ok()?;
                        Some(RobotInteractionUserData::Figures2([a, b]))
                    } else {
                        None
                    }
                }
                0x0103 => {
                    if user_data.len() == 75 {
                        let figs = [
                            InteractionFigure::try_from(&user_data[0..15]).ok()?,
                            InteractionFigure::try_from(&user_data[15..30]).ok()?,
                            InteractionFigure::try_from(&user_data[30..45]).ok()?,
                            InteractionFigure::try_from(&user_data[45..60]).ok()?,
                            InteractionFigure::try_from(&user_data[60..75]).ok()?,
                        ];
                        Some(RobotInteractionUserData::Figures5(figs))
                    } else {
                        None
                    }
                }
                0x0104 => {
                    if user_data.len() == 105 {
                        let figs: [InteractionFigure; 7] = [
                            InteractionFigure::try_from(&user_data[0..15]).ok()?,
                            InteractionFigure::try_from(&user_data[15..30]).ok()?,
                            InteractionFigure::try_from(&user_data[30..45]).ok()?,
                            InteractionFigure::try_from(&user_data[45..60]).ok()?,
                            InteractionFigure::try_from(&user_data[60..75]).ok()?,
                            InteractionFigure::try_from(&user_data[75..90]).ok()?,
                            InteractionFigure::try_from(&user_data[90..105]).ok()?,
                        ];
                        Some(RobotInteractionUserData::Figures7(figs))
                    } else {
                        None
                    }
                }
                0x0110 => {
                    if user_data.len() == 45 {
                        let fig = InteractionFigure::try_from(&user_data[0..15]).ok()?;
                        let mut data = [0u8; 30];
                        data.copy_from_slice(&user_data[15..45]);
                        Some(RobotInteractionUserData::CustomCharacter { figure: fig, data })
                    } else {
                        None
                    }
                }
                0x0120 => self
                    .as_sentry_cmd()
                    .map(RobotInteractionUserData::SentryCmd),
                0x0121 => self.as_radar_cmd().map(RobotInteractionUserData::RadarCmd),
                _ => None,
            }
        } else {
            None
        }
    }

    // 若为 0x0200~0x02FF 机器人间通信，返回原始负载
    pub fn as_robot_comm_raw(&self) -> Option<&[u8]> {
        if let RefereeFrameCmdData::RobotInteractionData {
            data_cmd_id,
            user_data,
            ..
        } = self
        {
            if (0x0200..=0x02FF).contains(data_cmd_id) {
                return Some(user_data.as_slice());
            }
        }
        None
    }
}

// 附录二：机器人 ID（u8 版本，用于 1 字节字段）
#[deku_derive(DekuRead, DekuWrite)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[deku(id_type = "u8")]
pub enum RobotID {
    // Red side 1...11
    #[deku(id = "1")]
    RedHero = 1,
    #[deku(id = "2")]
    RedEngineer = 2,
    #[deku(id = "3")]
    RedInfantry3 = 3,
    #[deku(id = "4")]
    RedInfantry4 = 4,
    #[deku(id = "5")]
    RedInfantry5 = 5,
    #[deku(id = "6")]
    RedAerial = 6,
    #[deku(id = "7")]
    RedSentry = 7,
    #[deku(id = "8")]
    RedDart = 8,
    #[deku(id = "9")]
    RedRadar = 9,
    #[deku(id = "10")]
    RedOutpost = 10,
    #[deku(id = "11")]
    RedBase = 11,
    // Blue side 101...111
    #[deku(id = "101")]
    BlueHero = 101,
    #[deku(id = "102")]
    BlueEngineer = 102,
    #[deku(id = "103")]
    BlueInfantry3 = 103,
    #[deku(id = "104")]
    BlueInfantry4 = 104,
    #[deku(id = "105")]
    BlueInfantry5 = 105,
    #[deku(id = "106")]
    BlueAerial = 106,
    #[deku(id = "107")]
    BlueSentry = 107,
    #[deku(id = "108")]
    BlueDart = 108,
    #[deku(id = "109")]
    BlueRadar = 109,
    #[deku(id = "110")]
    BlueOutpost = 110,
    #[deku(id = "111")]
    BlueBase = 111,
}

// 附录二：端点 ID（u16 版本，包含机器人 ID、选手端 ID、服务器 ID）
#[deku_derive(DekuRead, DekuWrite)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[deku(id_type = "u16", endian = "little")]
pub enum EndpointId {
    // Player Clients
    #[deku(id = "0x0101")]
    RedHeroClient = 0x0101,
    #[deku(id = "0x0102")]
    RedEngineerClient = 0x0102,
    #[deku(id = "0x0103")]
    RedInfantry3Client = 0x0103,
    #[deku(id = "0x0104")]
    RedInfantry4Client = 0x0104,
    #[deku(id = "0x0105")]
    RedInfantry5Client = 0x0105,
    #[deku(id = "0x0106")]
    RedAerialClient = 0x0106,
    #[deku(id = "0x0165")]
    BlueHeroClient = 0x0165,
    #[deku(id = "0x0166")]
    BlueEngineerClient = 0x0166,
    #[deku(id = "0x0167")]
    BlueInfantry3Client = 0x0167,
    #[deku(id = "0x0168")]
    BlueInfantry4Client = 0x0168,
    #[deku(id = "0x0169")]
    BlueInfantry5Client = 0x0169,
    #[deku(id = "0x016A")]
    BlueAerialClient = 0x016A,
    // Referee Server (for autonomous decision cmds)
    #[deku(id = "0x8080")]
    Server = 0x8080,
}
#[deku_derive(DekuRead, DekuWrite)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[deku(id_type = "u8", bits = 4)]
pub enum GameType {
    /// RoboMaster 机甲大师超级对抗赛
    #[deku(id = "1")]
    RMUC = 1,
    /// RoboMaster 机甲大师高校单项赛
    #[deku(id = "2")]
    RMUT = 2,
    /// ICRA RoboMaster 高校人工智能挑战赛
    #[deku(id = "3")]
    RMUA = 3,
    /// RoboMaster 机甲大师高校联盟赛 3V3 对抗
    #[deku(id = "4")]
    RMUL3v3 = 4,
    /// RoboMaster 机甲大师高校联盟赛步兵对抗
    #[deku(id = "5")]
    RMULInfantry = 5,
}

#[deku_derive(DekuRead, DekuWrite)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[deku(id_type = "u8", bits = 4)]
pub enum GameProgress {
    #[deku(id = "0")]
    NotStarted,
    #[deku(id = "1")]
    Preparation,
    #[deku(id = "2")]
    RefereeSelfCheck15s,
    #[deku(id = "3")]
    CountDown5s,
    #[deku(id = "4")]
    InProgress,
    #[deku(id = "5")]
    End,
}

#[deku_derive(DekuRead, DekuWrite)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[deku(id_type = "u8")]
pub enum GameResult {
    #[deku(id = "0")]
    Draw,
    #[deku(id = "1")]
    RedWin,
    #[deku(id = "2")]
    BlueWin,
}

#[deku_derive(DekuRead, DekuWrite)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[deku(id_type = "u8")]
pub enum RefereeWarningLevel {
    #[deku(id = "1")]
    BothYellow = 1,
    #[deku(id = "2")]
    Yellow = 2,
    #[deku(id = "3")]
    Red = 3,
    #[deku(id = "4")]
    Forfeit = 4,
}

#[deku_derive(DekuRead, DekuWrite)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]

pub struct EventData {
    #[deku(bits = 1)]
    pub b0: bool,
    #[deku(bits = 1)]
    pub b1: bool,
    #[deku(bits = 1)]
    pub b2: bool,
    #[deku(bits = 1)]
    pub b3: bool,
    #[deku(bits = 1)]
    pub b4: bool,
    // #[deku(bits = 2)]
    pub b5_6: OccupyType,
    pub b7_8: OccupyType,
    #[deku(bits = 9)]
    pub b9_17: u16,
    #[deku(bits = 3)]
    pub b18_20: u8,
    pub b21_22: OccupyType,
    pub b23_24: OccupyType,
    #[deku(bits = 7, temp, temp_value = "u8::default()")]
    pub b25_31: u8,
}

#[deku_derive(DekuRead, DekuWrite)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[deku(id_type = "u8", bits = 2)]
pub enum OccupyType {
    #[deku(id = "0")]
    None = 0,
    #[deku(id = "1")]
    Our = 1,
    #[deku(id = "2")]
    Enemy = 2,
    #[deku(id = "3")]
    Both = 3,
}

#[deku_derive(DekuRead, DekuWrite)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[deku(id_type = "u8", bits = 3)]
pub enum DartTarget {
    #[deku(id = "0")]
    None = 0,
    #[deku(id = "1")]
    Outpost = 1,
    #[deku(id = "2")]
    BaseFixed = 2,
    #[deku(id = "3")]
    BaseRandomFixed = 3,
    #[deku(id = "4")]
    BaseRandomMoving = 4,
}

#[deku_derive(DekuRead, DekuWrite)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[deku(id_type = "u8", bits = 2)]
pub enum DartSelectionTarget {
    #[deku(id = "0")]
    NoneOrOutpost = 0,
    #[deku(id = "1")]
    BaseFixed = 1,
    #[deku(id = "2")]
    BaseRandomFixed = 2,
    #[deku(id = "3")]
    BaseRandomMoving = 3,
}

#[deku_derive(DekuRead, DekuWrite)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[deku(id_type = "u8", bits = 4)]
pub enum HpDeductionReason {
    #[deku(id = "0")]
    ArmorHitByProjectile = 0,
    #[deku(id = "1")]
    ModuleOffline = 1,
    #[deku(id = "5")]
    ArmorCollision = 5,
}
#[deku_derive(DekuRead, DekuWrite)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[deku(id_type = "u8")]
pub enum BulletType {
    #[deku(id = "1")]
    Bullet17mm = 1,
    #[deku(id = "2")]
    Bullet42mm = 2,
}

#[deku_derive(DekuRead, DekuWrite)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[deku(id_type = "u8")]
pub enum ShooterNumber {
    #[deku(id = "1")]
    Shooter17mm1 = 1,
    #[deku(id = "2")]
    Shooter17mm2 = 2,
    #[deku(id = "3")]
    Shooter42mm = 3,
}
