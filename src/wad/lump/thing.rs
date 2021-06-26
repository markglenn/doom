use byteorder::LittleEndian as le;
use byteorder::ReadBytesExt;
use num_traits::FromPrimitive;
use std::io::Cursor;

use std::io;

bitflags! {
    struct ThingFlags: u16 {
        const SKILL_1_AND_2     = 0x0001;
        const SKILL_3           = 0x0002;
        const SKILL_4_AND_5	    = 0x0004;
        const IS_DEAF           = 0x0008;
        const MULTIPLAYER_ONLY  = 0x0010;
        const NOT_IN_DEATHMATCH = 0x0020;
        const NOT_IN_COOP       = 0x0040;
    }
}

#[derive(FromPrimitive, ToPrimitive, Debug)]
pub enum ThingType {
    Player1Start = 1,
    Player2Start = 2,
    Player3Start = 3,
    Player4Start = 4,
    Bluecard = 5,
    Yellowcard = 6,
    SpiderMastermind = 7,
    Backpack = 8,
    ShotgunGuy = 9,
    GibbedMarine = 10,
    DeathmatchStart = 11,
    GibbedMarineExtra = 12,
    Redcard = 13,
    TeleportDest = 14,
    DeadMarine = 15,
    Cyberdemon = 16,
    CellPack = 17,
    DeadZombieMan = 18,
    DeadShotgunGuy = 19,
    DeadImp = 20,
    DeadDemon = 21,
    DeadCacodemon = 22,
    DeadLostSoul = 23,
    Gibs = 24,
    DeadStick = 25,
    LiveStick = 26,
    HeadOnAStick = 27,
    HeadsOnAStick = 28,
    HeadCandles = 29,
    TallGreenColumn = 30,
    ShortGreenColumn = 31,
    TallRedColumn = 32,
    ShortRedColumn = 33,
    Candlestick = 34,
    Candelabra = 35,
    HeartColumn = 36,
    SkullColumn = 37,
    RedSkull = 38,
    YellowSkull = 39,
    BlueSkull = 40,
    EvilEye = 41,
    FloatingSkull = 42,
    TorchTree = 43,
    BlueTorch = 44,
    GreenTorch = 45,
    RedTorch = 46,
    Stalagtite = 47,
    TechPillar = 48,
    BloodyTwitch = 49,
    Meat2 = 50,
    Meat3 = 51,
    Meat4 = 52,
    Meat5 = 53,
    BigTree = 54,
    ShortBlueTorch = 55,
    ShortGreenTorch = 56,
    ShortRedTorch = 57,
    Spectre = 58,
    NonsolidMeat2 = 59,
    NonsolidMeat4 = 60,
    NonsolidMeat3 = 61,
    NonsolidMeat5 = 62,
    NonsolidTwitch = 63,

    Shotgun = 2001,
    Chaingun = 2002,
    RocketLauncher = 2003,
    PlasmaRifle = 2004,
    Chainsaw = 2005,
    BFG9000 = 2006,
    Clip = 2007,
    Shell = 2008,
    RocketAmmo = 2010,
    Stimpack = 2011,
    Medikit = 2012,
    Soulsphere = 2013,
    HealthBonus = 2014,
    ArmorBonus = 2015,
    GreenArmor = 2018,
    BlueArmor = 2019,

    InvulnerabilitySphere = 2022,
    Berserk = 2023,
    BlurSphere = 2024,
    RadSuit = 2025,
    Allmap = 2026,
    Column = 2028,
    ExplosiveBarrel = 2035,

    Infrared = 2045,
    RocketBox = 2046,
    Cell = 2047,
    ClipBox = 2048,
    ShellBox = 2049,

    Imp = 3001,
    Demon = 3002,
    BaronOfHell = 3003,
    ZombieMan = 3004,
    Cacodemon = 3005,
    LostSoul = 3006,
}

#[derive(Debug)]
pub struct Thing {
    x_position: i16,
    y_position: i16,
    angle: u16,
    thing_type: ThingType,
    flags: ThingFlags,
}

impl Thing {
    pub fn load(buffer: Vec<u8>) -> io::Result<Vec<Thing>> {
        let num_things = buffer.len() / 10;
        let mut stream = Cursor::new(buffer);

        let mut things = Vec::with_capacity(num_things);

        for _ in 0..num_things {
            let x_position = stream.read_i16::<le>()?;
            let y_position = stream.read_i16::<le>()?;
            let angle = stream.read_u16::<le>()?;
            let thing_type = stream.read_u16::<le>()?;

            let flags = stream.read_u16::<le>()?;

            things.push(Thing {
                x_position,
                y_position,
                angle,
                thing_type: FromPrimitive::from_u16(thing_type).unwrap(),
                flags: ThingFlags::from_bits(flags).unwrap(),
            })
        }

        Ok(things)
    }
}
