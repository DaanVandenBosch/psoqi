use std::io::{Read, Seek};
use read;
use read::qst::{self, QstFile};
use types::Quest;

// High level read method that delegates to the correct lower level read methods.
pub fn read<T: Read + Seek>(data: &mut T) -> read::Result<Quest> {
    let QstFile { dat, bin } = qst::read(data)?;

    return Ok(Quest {
        name: bin.quest_name,
        short_description: bin.short_description,
        episode: dat.episode,
        monster_counts: dat.monster_counts,
    });
}
