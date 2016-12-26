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
        let mut result = None;

        result = match (id, skin % 3, episode) {
            (0x044, 0, _) => Some(Booma),
            (0x044, 1, _) => Some(Gobooma),
            (0x044, 2, _) => Some(Gigobooma),

            (0x063, 0, _) => Some(EvilShark),
            (0x063, 1, _) => Some(PalShark),
            (0x063, 2, _) => Some(GuilShark),

            (0x0A6, 0, Episode::I) => Some(Dimenian),
            (0x0A6, 0, Episode::II) => Some(Dimenian2),
            (0x0A6, 1, Episode::I) => Some(LaDimenian),
            (0x0A6, 1, Episode::II) => Some(LaDimenian2),
            (0x0A6, 2, Episode::I) => Some(SoDimenian),
            (0x0A6, 2, Episode::II) => Some(SoDimenian2),

            (0x0D6, 0, _) => Some(Mericarol),
            (0x0D6, 1, _) => Some(Mericus),
            (0x0D6, 2, _) => Some(Merikle),

            (0x115, 0, _) => Some(Boota),
            (0x115, 1, _) => Some(ZeBoota),
            (0x115, 2, _) => Some(BaBoota),
            (0x117, 0, _) => Some(Goran),
            (0x117, 1, _) => Some(PyroGoran),
            (0x117, 2, _) => Some(GoranDetonator),

            _ => result
        };

        result = match (id, skin % 2, episode) {
            (0x040, 0, Episode::I) => Some(Hildebear),
            (0x040, 0, Episode::II) => Some(Hildebear2),
            (0x040, 1, Episode::I) => Some(Hildeblue),
            (0x040, 1, Episode::II) => Some(Hildeblue2),
            (0x041, 0, Episode::I) => Some(RagRappy),
            (0x041, 0, Episode::II) => Some(RagRappy2),
            (0x041, 0, Episode::IV) => Some(SandRappy),
            (0x041, 1, Episode::I) => Some(AlRappy),
            (0x041, 1, Episode::II) => Some(LoveRappy),
            (0x041, 1, Episode::IV) => Some(DelRappy),

            (0x061, 0, Episode::I) => Some(if area > 15 { DelLily } else { PoisonLily }),
            (0x061, 0, Episode::II) => Some(if area > 15 { DelLily } else { PoisonLily2 }),
            (0x061, 1, Episode::I) => Some(if area > 15 { DelLily } else { NarLily }),
            (0x061, 1, Episode::II) => Some(if area > 15 { DelLily } else { NarLily2 }),

            (0x080, 0, Episode::I) => Some(Dubchic),
            (0x080, 0, Episode::II) => Some(Dubchic2),
            (0x080, 1, Episode::I) => Some(Gilchic),
            (0x080, 1, Episode::II) => Some(Gilchic2),

            (0x0D4, 0, _) => Some(SinowBerill),
            (0x0D4, 1, _) => Some(SinowSpigell),
            (0x0D5, 0, _) => Some(Merillia),
            (0x0D5, 1, _) => Some(Meriltas),
            (0x0D7, 0, _) => Some(UlGibbon),
            (0x0D7, 1, _) => Some(ZolGibbon),

            (0x0DD, 0, _) => Some(Dolmolm),
            (0x0DD, 1, _) => Some(Dolmdarl),
            (0x0E0, 0, _) => Some(if area > 15 { Epsilon } else { SinowZoa }),
            (0x0E0, 1, _) => Some(if area > 15 { Epsilon } else { SinowZele }),

            (0x112, 0, _) => Some(MerissaA),
            (0x112, 1, _) => Some(MerissaAA),
            (0x114, 0, _) => Some(Zu),
            (0x114, 1, _) => Some(Pazuzu),
            (0x116, 0, _) => Some(Dorphon),
            (0x116, 1, _) => Some(DorphonEclair),
            (0x119, 0, _) => Some(if regular { SaintMillion } else { Kondrieu }),
            (0x119, 1, _) => Some(if regular { Shambertin } else { Kondrieu }),

            _ => result
        };

        result = match (id, episode) {
            (0x042, Episode::I) => Some(Monest),
            (0x042, Episode::II) => Some(Monest2),
            (0x043, Episode::I) => Some(if regular { SavageWolf } else { BarbarousWolf }),
            (0x043, Episode::II) => Some(if regular { SavageWolf2 } else { BarbarousWolf2 }),

            (0x060, Episode::I) => Some(GrassAssassin),
            (0x060, Episode::II) => Some(GrassAssassin2),
            (0x062, _) => Some(NanoDragon),
            (0x064, _) => Some(if regular { PofuillySlime } else { PouillySlime }),
            (0x065, Episode::I) => Some(PanArms),
            (0x065, Episode::II) => Some(PanArms2),

            (0x081, Episode::I) => Some(Garanz),
            (0x081, Episode::II) => Some(Garanz2),
            (0x082, _) => Some(if regular { SinowBeat } else { SinowGold }),
            (0x083, _) => Some(Canadine),
            (0x084, _) => {
                *dat_file.monster_counts.entry(Canadine).or_insert(0) += 8;
                Some(Canane)
            },
            (0x085, Episode::I) => Some(Dubswitch),
            (0x085, Episode::II) => Some(Dubswitch2),

            (0x0A0, Episode::I) => Some(Delsaber),
            (0x0A0, Episode::II) => Some(Delsaber2),
            (0x0A1, Episode::I) => Some(ChaosSorcerer),
            (0x0A1, Episode::II) => Some(ChaosSorcerer2),
            (0x0A2, _) => Some(DarkGunner),
            (0x0A4, _) => Some(ChaosBringer),
            (0x0A5, Episode::I) => Some(DarkBelra),
            (0x0A5, Episode::II) => Some(DarkBelra2),
            (0x0A7, _) => Some(Bulclaw),
            (0x0A8, _) => Some(Claw),

            (0x0C0, Episode::I) => Some(Dragon),
            (0x0C0, Episode::II) => Some(GalGryphon),
            (0x0C1, _) => Some(DeRolLe),
            (0x0C5, _) => Some(VolOpt),
            (0x0C8, _) => Some(DarkFalz),
            (0x0CA, _) => Some(OlgaFlow),
            (0x0CB, _) => Some(BarbaRay),
            (0x0CC, _) => Some(GolDragon),

            (0x0D8, _) => Some(Gibbles),
            (0x0D9, _) => Some(Gee),
            (0x0DA, _) => Some(GiGue),

            (0x0DB, _) => Some(Deldepth),
            (0x0DC, _) => Some(Delbiter),
            (0x0DE, _) => Some(Morfos),
            (0x0DF, _) => Some(Recobox),
            (0x0E1, _) => Some(IllGill),

            (0x110, _) => Some(Astark),
            (0x111, _) => Some(if regular { SatelliteLizard } else { Yowie }),
            (0x113, _) => Some(Girtablulu),

            _ => result
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
