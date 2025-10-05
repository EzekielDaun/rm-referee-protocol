use deku::deku_derive;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// 0x0301 子内容：哨兵自主决策指令（0x0120），位域结构（4 字节）
#[deku_derive(DekuRead, DekuWrite)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SentryCmd {
    #[deku(bits = 1)]
    pub confirm_revive: bool,
    #[deku(bits = 1)]
    pub confirm_immediate_revive: bool,
    // bit 2-12：哨兵将要兑换的发弹量值（单调递增）
    #[deku(bits = 11)]
    pub exchange_projectile_allowance: u16,
    // bit 13-16：远程兑换发弹量的请求次数（单调递增，每次 +1）
    #[deku(bits = 4)]
    pub remote_exchange_projectile_count: u8,
    // bit 17-20：远程兑换血量的请求次数（单调递增，每次 +1）
    #[deku(bits = 4)]
    pub remote_exchange_hp_count: u8,
    // bit 21-31：保留
    #[deku(bits = 11, temp, temp_value = "u16::default()")]
    pub reserved: u16,
}

// 0x0301 子内容：雷达自主决策指令（0x0121），单字节计数（1 字节）
#[deku_derive(DekuRead, DekuWrite)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RadarCmd {
    pub radar_cmd: u8,
}

// 0x0301 子内容：图形/图层相关类型
#[deku_derive(DekuRead, DekuWrite)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[deku(id_type = "u8")]
pub enum LayerDeleteType {
    #[deku(id = "0")]
    None = 0,
    #[deku(id = "1")]
    DeleteLayer = 1,
    #[deku(id = "2")]
    DeleteAll = 2,
}

#[deku_derive(DekuRead, DekuWrite)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InteractionLayerDelete {
    pub delete_type: LayerDeleteType,
    pub layer: u8,
}

#[deku_derive(DekuRead, DekuWrite)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[deku(id_type = "u8", bits = 3)]
pub enum FigureOperateType {
    #[deku(id = "0")]
    None,
    #[deku(id = "1")]
    Add,
    #[deku(id = "2")]
    Modify,
    #[deku(id = "3")]
    Delete,
}

#[deku_derive(DekuRead, DekuWrite)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[deku(id_type = "u8", bits = 3)]
pub enum FigureType {
    #[deku(id = "0")]
    Line,
    #[deku(id = "1")]
    Rectangle,
    #[deku(id = "2")]
    Circle,
    #[deku(id = "3")]
    Ellipse,
    #[deku(id = "4")]
    Arc,
    #[deku(id = "5")]
    Float,
    #[deku(id = "6")]
    Int,
    #[deku(id = "7")]
    Char,
}

#[deku_derive(DekuRead, DekuWrite)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[deku(id_type = "u8", bits = 4)]
pub enum FigureColor {
    #[deku(id = "0")]
    SelfColor,
    #[deku(id = "1")]
    Yellow,
    #[deku(id = "2")]
    Green,
    #[deku(id = "3")]
    Orange,
    #[deku(id = "4")]
    Magenta,
    #[deku(id = "5")]
    Pink,
    #[deku(id = "6")]
    Cyan,
    #[deku(id = "7")]
    Black,
    #[deku(id = "8")]
    White,
}

// 单个图形（15 字节）：3 字节图形名 + 三段 32bit 位域
#[deku_derive(DekuRead, DekuWrite)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InteractionFigure {
    pub figure_name: [u8; 3],
    // 配置 1（32bit）
    pub operate_type: FigureOperateType,
    pub figure_type: FigureType,
    #[deku(bits = 4)]
    pub layer: u8,
    pub color: FigureColor,
    #[deku(bits = 9)]
    pub details_a: u16,
    #[deku(bits = 9)]
    pub details_b: u16,
    // 配置 2（32bit）
    #[deku(bits = 10)]
    pub width: u16,
    #[deku(bits = 11)]
    pub start_x: u16,
    #[deku(bits = 11)]
    pub start_y: u16,
    // 配置 3（32bit）
    #[deku(bits = 10)]
    pub details_c: u16,
    #[deku(bits = 11)]
    pub details_d: u16,
    #[deku(bits = 11)]
    pub details_e: u16,
}

// 0x0301 子内容枚举：固定长度的内联为枚举
#[deku_derive(DekuRead, DekuWrite)]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[deku(id_type = "u16", ctx = "id: u16, len: usize")]
pub enum RobotInteractionUserData {
    // 0x0100: 2 字节 删除图层
    #[deku(id = "0x0100")]
    LayerDelete(InteractionLayerDelete),
    // 0x0101: 15 字节 单个图形
    #[deku(id = "0x0101")]
    Figure(InteractionFigure),
    // 0x0102: 30 字节 两个图形
    #[deku(id = "0x0102")]
    Figures2([InteractionFigure; 2]),
    // 0x0103: 75 字节 五个图形
    #[deku(id = "0x0103")]
    Figures5([InteractionFigure; 5]),
    // 0x0104: 105 字节 七个图形
    #[deku(id = "0x0104")]
    Figures7([InteractionFigure; 7]),
    // 0x0110: 45 字节 字符图形（图形配置 + 30 字节内容）
    #[deku(id = "0x0110")]
    CustomCharacter {
        figure: InteractionFigure,
        data: [u8; 30],
    },
    // 0x0120: 4 字节
    #[deku(id = "0x0120")]
    SentryCmd(SentryCmd),
    // 0x0121: 1 字节
    #[deku(id = "0x0121")]
    RadarCmd(RadarCmd),
}
