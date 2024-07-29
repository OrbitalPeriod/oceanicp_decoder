mod models;

use models::dives::{Dive, Dives};
use models::file::InputFile;
use serde_xml_rs::{from_reader, from_str};
use std::fs::{read, File};
use std::io::{self, BufRead, BufReader};

fn main() {
    let file_path = "dive_data.uddf";
    let file = File::open(file_path).expect("Failed to read file");

    let reader = BufReader::new(file);

    let xml : InputFile = from_reader(reader).unwrap();

    let dives : Vec<f32> = xml.dives.repitition_group.iter().flat_map(|group| &group.dives).flat_map(|x| x.calculate_sac_rate()).collect();
    println!("{:?}", dives);
}


//todo: Add the Optional note field with rating and para
//normalize everything to use SI units
//Organize models some more
//Set things up to output to different format