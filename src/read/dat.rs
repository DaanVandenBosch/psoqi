use std::collections::BTreeMap;
use std::io::{Read, Seek, SeekFrom};
use byteorder::{ReadBytesExt, LittleEndian};
use read;
use types::{Episode, MonsterType};

// A .dat file describes objects, enemies and more.
pub struct DatFile {
    pub episode: Episode,
    pub monster_counts: BTreeMap<MonsterType, u32>
}

// Low level read method for .dat files.
pub fn read<T: Read + Seek>(data: &mut T) -> read::Result<DatFile> {
    use types::MonsterType::*;

    let monster_data = extract_monster_data(data)?;
    let episode = detect_episode(&monster_data).unwrap_or(Episode::I);

    let mut dat_file = DatFile {
        episode: episode,
        monster_counts: BTreeMap::new()
    };

    for MonsterData { id, regular, skin, area } in monster_data {
        let result = match (id, skin) {
            (0x0040, 0) => Some(if episode == Episode::I { Hildebear } else { Hildebear2 }),
            (0x0040, 1) => Some(if episode == Episode::I { Hildeblue } else { Hildeblue2 }),
            (0x0041, 0) => Some(match episode {
                Episode::I => RagRappy,
                Episode::II => RagRappy2,
                Episode::IV => SandRappy
            }),
            (0x0041, 1) => Some(match episode {
                Episode::I => AlRappy,
                Episode::II => AlRappy2,
                Episode::IV => DelRappy
            }),
            (0x0042, _) => Some(if episode == Episode::I { Monest } else { Monest2 }),
            (0x0043, 0) => Some(if episode == Episode::I { SavageWolf } else { SavageWolf2 }),
            (0x0043, 1) => Some(if episode == Episode::I { BarbarousWolf } else { BarbarousWolf2 }),
            (0x0044, 0) => Some(Booma),
            (0x0044, 1) => Some(Gobooma),
            (0x0044, 2) => Some(Gigobooma),

            (0x0060, _) => Some(if episode == Episode::I { GrassAssassin } else { GrassAssassin2 }),
            (0x0061, 0) => Some(if area > 15 { DelLily } else if episode == Episode::I { PoisonLily } else { PoisonLily2 }),
            (0x0061, 1) => Some(if area > 15 { DelLily } else if episode == Episode::I { NarLily } else { NarLily2 }),
            (0x0062, _) => Some(NanoDragon),
            (0x0063, 0) => Some(EvilShark),
            (0x0063, 1) => Some(PalShark),
            (0x0063, 2) => Some(GuilShark),
            (0x0064, _) => Some(if regular { PofuillySlime } else { PouillySlime }),
            (0x0065, _) => Some(if episode == Episode::I { PanArms } else { PanArms2 }),

            (0x0080, 0) => Some(if episode == Episode::I { Dubchic } else { Dubchic2 }),
            (0x0080, 1) => Some(if episode == Episode::I { Gilchic } else { Gilchic2 }),
            (0x0081, _) => Some(if episode == Episode::I { Garanz } else { Garanz2 }),
            (0x0082, _) => Some(if regular { SinowBeat } else { SinowGold }),
            (0x0083, _) => Some(Canadine),
            (0x0084, _) => {
                *dat_file.monster_counts.entry(Canadine).or_insert(0) += 8;
                Some(Canane)
            },
            (0x0085, _) => Some(if episode == Episode::I { Dubwitch } else { Dubwitch2 }),

            (0x00A0, _) => Some(if episode == Episode::I { Delsaber } else { Delsaber2 }),
            (0x00A1, _) => Some(if episode == Episode::I { ChaosSorcerer } else { ChaosSorcerer2 }),
            (0x00A2, _) => Some(DarkGunner),
            (0x00A4, _) => Some(ChaosBringer),
            (0x00A5, _) => Some(if episode == Episode::I { DarkBelra } else { DarkBelra2 }),
            (0x00A6, 0) => Some(if episode == Episode::I { Dimenian } else { Dimenian2 }),
            (0x00A6, 1) => Some(if episode == Episode::I { LaDimenian } else { LaDimenian2 }),
            (0x00A6, 2) => Some(if episode == Episode::I { SoDimenian } else { SoDimenian2 }),
            (0x00A7, _) => Some(Bulclaw),
            (0x00A8, _) => Some(Claw),

            (0x00C0, _) => Some(if episode == Episode::I { Dragon } else { GalGryphon }),
            (0x00C1, _) => Some(DeRolLe),
            (0x00C5, _) => Some(VolOpt),
            (0x00C8, _) => Some(DarkFalz),
            (0x00CA, _) => Some(OlgaFlow),
            (0x00CB, _) => Some(BarbaRay),
            (0x00CC, _) => Some(GolDragon),

            (0x00D4, 0) => Some(SinowBerill),
            (0x00D4, 1) => Some(SinowSpigell),
            (0x00D5, 0) => Some(Merillia),
            (0x00D5, 1) => Some(Meriltas),
            (0x00D6, 0) => Some(Mericarol),
            (0x00D6, 1) => Some(Mericus),
            (0x00D6, 2) => Some(Merikle),
            (0x00D7, 0) => Some(UlGibbon),
            (0x00D7, 1) => Some(ZolGibbon),
            (0x00D8, _) => Some(Gibbles),
            (0x00D9, _) => Some(Gee),
            (0x00DA, _) => Some(GiGue),

            (0x00DB, _) => Some(Deldepth),
            (0x00DC, _) => Some(Delbiter),
            (0x00DD, 0) => Some(Dolmolm),
            (0x00DD, 1) => Some(Dolmdarl),
            (0x00DE, _) => Some(Morfos),
            (0x00DF, _) => Some(Recobox),
            (0x00E0, 0) => Some(if area > 15 { Epsilon } else { SinowZoa }),
            (0x00E0, 1) => Some(if area > 15 { Epsilon } else { SinowZele }),
            (0x00E1, _) => Some(IllGill),

            (0x0110, _) => Some(Astark),
            (0x0111, 0) => Some(SatelliteLizard),
            (0x0111, 1) => Some(Yowie),
            (0x0112, 0) => Some(MerissaA),
            (0x0112, 1) => Some(MerissaAA),
            (0x0113, _) => Some(Girtablulu),
            (0x0114, 0) => Some(Zu),
            (0x0114, 1) => Some(Pazuzu),
            (0x0115, 0) => Some(Boota),
            (0x0115, 1) => Some(ZaBoota),
            (0x0115, 2) => Some(BaBoota),
            (0x0116, 0) => Some(Dorphon),
            (0x0116, 1) => Some(DorphonEclair),
            (0x0117, 0) => Some(Goran),
            (0x0117, 1) => Some(PyroGoran),
            (0x0117, 2) => Some(GoranDetonator),
            (0x0119, 0) => Some(SaintMillion),
            (0x0119, 1) => Some(Shambertin),

            _ => None
        };

        if let Some(monster_type) = result {
            *dat_file.monster_counts.entry(monster_type).or_insert(0) += 1;
        }
    }

    return Ok(dat_file);
}

struct MonsterData {
    id: u32,
    regular: bool,
    skin: u32,
    area: u32
}

fn detect_episode(data: &Vec<MonsterData>) -> Option<Episode> {
    for ref monster_data in data {
        let area = monster_data.area;

        match monster_data.id {
            0x44 | 0x62 ... 0x64 | 0x82 ... 0x84 | 0xA2 | 0xA4 | 0xA7 ... 0xA8 | 0xC1 | 0xC5 | 0xC8 =>
                return Some(Episode::I),
            0xCA ... 0xE1 =>
                return Some(Episode::II),
            0x110 ... 0x119 =>
                return Some(Episode::IV),
            0x40 =>
                if area == 1 { return Some(Episode::II) },
            0x41 =>
                if area >= 6 { return Some(Episode::IV) },
            0x43 =>
                return Some(if area <= 2 { Episode::I } else { Episode::II }),
            0x60 | 0xA5 ... 0xA6 =>
                return Some(if area <= 2 { Episode::II } else { Episode::I }),
            0x61 =>
                return Some(if 3 <= area && area <= 5 { Episode::I } else { Episode::II }),
            0x80 ... 0x81 | 0x85 | 0xA0 ... 0xA1 =>
                return Some(if area <= 4 { Episode::II } else { Episode::I }),
            _ =>
                {}
        }
    }

    return None;
}

fn extract_monster_data<T: Read + Seek>(data: &mut T) -> read::Result<Vec<MonsterData>> {
    let mut pos = 0;
    let mut vec = Vec::new();

    while data.seek(SeekFrom::Start(pos)).is_ok() {
        let object_type = data.read_u32::<LittleEndian>()?;
        let next_header = data.read_u32::<LittleEndian>()?;
        let area = data.read_u32::<LittleEndian>()?;
        let size = data.read_u32::<LittleEndian>()?;

        if object_type == 2 {
            let monster_count = size / 72;

            for _ in 0..monster_count {
                let id = data.read_u32::<LittleEndian>()?;
                data.seek(SeekFrom::Current(44))?;
                let regular = data.read_u32::<LittleEndian>()? & 0x800000 == 0;
                data.seek(SeekFrom::Current(12))?;
                let skin = data.read_u32::<LittleEndian>()?;
                data.seek(SeekFrom::Current(4))?;

                vec.push(MonsterData {
                    id: id,
                    regular: regular,
                    skin: skin,
                    area: area
                });
            }
        } else if object_type != 1 && object_type != 3 {
            break;
        }

        pos += next_header as u64;
    }

    return Ok(vec);
}
