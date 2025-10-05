use deku::deku_derive;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// IDs
#[deku_derive(DekuRead, DekuWrite)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[deku(id_type = "u8")]
pub enum RobotID {
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

#[deku_derive(DekuRead, DekuWrite)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[deku(id_type = "u16", endian = "little")]
pub enum EndpointID {
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
    #[deku(id = "0x8080")]
    Server = 0x8080,
}

// Common enums
#[deku_derive(DekuRead, DekuWrite)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[deku(id_type = "u8", bits = 4)]
pub enum GameType {
    #[deku(id = "1")]
    RMUC = 1,
    #[deku(id = "2")]
    RMUT = 2,
    #[deku(id = "3")]
    RMUA = 3,
    #[deku(id = "4")]
    RMUL3v3 = 4,
    #[deku(id = "5")]
    RMULInfantry = 5,
}

#[deku_derive(DekuRead, DekuWrite)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[deku(id_type = "u8")]
pub enum BulletType {
    #[deku(id = "1")]
    Bullet17mm = 1,
    #[deku(id = "2")]
    Bullet42mm = 2,
}

#[deku_derive(DekuRead, DekuWrite)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[deku(id_type = "u8")]
pub enum ShooterNumber {
    #[deku(id = "1")]
    Shooter17mm1 = 1,
    #[deku(id = "2")]
    Shooter17mm2 = 2,
    #[deku(id = "3")]
    Shooter42mm = 3,
}
