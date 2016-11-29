extern crate encoding;
extern crate byteorder;
extern crate csv;
extern crate clap;

mod read;
mod prs;
mod types;
mod util;

use std::cmp::max;
use std::fs::File;
use std::io::{self, Write, BufReader};
use std::iter::Iterator;
use std::path::{Path, PathBuf};
use clap::{App, Arg};
use read::{quest, ReadError};
use types::{Quest, MonsterType};

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("csv")
            .long("csv")
            .short("c")
            .takes_value(false)
            .help("Output information in CSV format"))
        .arg(Arg::with_name("INPUT")
            .required(true)
            .multiple(true)
            .help("Files and/or directories to process"))
        .get_matches();

    let files: Vec<&str> = matches.values_of("INPUT").unwrap().collect();
    let files: Vec<&Path> = files.iter().map(|&arg| Path::new(arg)).collect();

    let mut quest_results = Vec::new();
    read_quests(&files, &mut quest_results);

    let mut quests = Vec::new();
    let mut errors = Vec::new();

    for result in quest_results {
        match result {
            Ok(quest) => quests.push(quest),
            Err(err) => errors.push(err)
        }
    }

    if matches.is_present("csv") {
        if quests_to_csv(&quests).is_err() {
            writeln!(&mut io::stderr(), "CSV generation failed.").unwrap();
        }
    } else {
        print_quests(&quests);
    }
}

fn read_quests(files: &Vec<&Path>, quests: &mut Vec<read::Result<Quest>>) {
    for file in files {
        if file.is_file() {
            quests.push(read_file(file));
        }
    }

    for file in files {
        if file.is_dir() {
            match file.read_dir() {
                Ok(sub_files) => {
                    let sub_paths: Vec<PathBuf> = sub_files.filter_map(|sf| sf.ok()).map(|sf| sf.path()).collect();
                    let sub_paths = sub_paths.iter().map(|sp| sp.as_path()).collect();
                    read_quests(&sub_paths, quests);
                },
                Err(err) => quests.push(Err(ReadError::from(err)))
            }
        }
    }
}

fn read_file(file_name: &Path) -> read::Result<Quest> {
    let file = File::open(file_name)?;
    let mut buf_reader = BufReader::new(file);
    return quest::read(&mut buf_reader);
}

fn quests_to_csv(quests: &Vec<Quest>) -> csv::Result<()> {
    let mut writer = csv::Writer::from_writer(io::stdout());

    let monster_types: Vec<String> = (0..MonsterType::Shambertin as u8 + 1)
        .map(|i| unsafe { std::mem::transmute::<u8, MonsterType>(i) }.to_string())
        .collect();
    writer.encode(("Quest", "Short Description", "Episode", monster_types))?;

    for &Quest { ref name, ref short_description, ref episode, ref monster_counts } in quests {
        let mut record = (name, short_description, episode.to_string(), vec![0; MonsterType::Shambertin as usize + 1]);

        for (monster_type, count) in monster_counts {
            record.3[*monster_type as usize] = *count;
        }

        writer.encode(record)?;
    }

    Ok(())
}

fn print_quests(quests: &Vec<Quest>) {
    for &Quest { ref name, ref short_description, ref episode, ref monster_counts } in quests {
        println!("Name: {}\nDetected episode {:?}.\nShort description:\n\n{}", name, episode, short_description);

        println!("\nMonster counts:");

        let max_count = monster_counts.values().fold(0, |acc, &count| { max(acc, count) }) as f64;
        let number_width = max_count.log10().round() as usize + 1;

        for (monster_type, count) in monster_counts {
            println!("{:>width$} {}", count, monster_type, width = number_width);
        }

        println!("\n");
    }
}
