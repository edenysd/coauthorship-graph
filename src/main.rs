mod reader;
mod types;
mod utils;

use crate::reader::read_publication_list;
use std::{
    fs::{create_dir, File},
    path::Path,
    process,
};

fn execute_calculations(pub_list: Vec<types::Publication>, path_dir: String) {
    println!("calculating exclusivity_per_pub");
    let exclusivity_per_pub = utils::calculations::calculate_exclusivity_per_pub(&pub_list);
    serde_json::to_writer_pretty(
        File::create(path_dir.clone() + "exclusivity_per_pub.json")
            .expect("Failed to create file."),
        &exclusivity_per_pub,
    )
    .expect("Failed to write file");
    println!("exclusivity_per_pub finished");

    println!("calculating co_authorship_freq");
    let co_authorship_freq =
        utils::calculations::calculate_co_authorship_freq(&exclusivity_per_pub);
    serde_json::to_writer_pretty(
        File::create(path_dir.clone() + "co_authorship_freq.json").expect("Failed to create file."),
        &co_authorship_freq,
    )
    .expect("Failed to write file");
    println!("co_authorship_freq finished");

    println!("calculating total_co_authorship_freq_per_author");
    let total_co_authorship_freq_per_author =
        utils::calculations::calculate_total_co_authorship_freq_per_author(&co_authorship_freq);
    serde_json::to_writer_pretty(
        File::create(path_dir.clone() + "total_co_authorship_freq_per_author.json")
            .expect("Failed to create file."),
        &total_co_authorship_freq_per_author,
    )
    .expect("Failed to write file");
    println!("total_co_authorship_freq_per_author finished");

    println!("calculating normalized_weights");
    let normalized_weights = utils::calculations::calculate_normalized_weights(
        &co_authorship_freq,
        &total_co_authorship_freq_per_author,
    );
    serde_json::to_writer_pretty(
        File::create(path_dir.clone() + "normalized_weights.json").expect("Failed to create file."),
        &normalized_weights,
    )
    .expect("Failed to write file");
    println!("normalized_weights finished");
}

fn main() {
    let path_dir = "./calculated-data/".to_string();
    if Path::new(&path_dir).exists() == false {
        create_dir(path_dir.clone()).expect("Failed to create the root dir");
    }

    let pub_list = match read_publication_list() {
        Err(err) => {
            println!("error running example: {}", err);
            process::exit(1);
        }
        Ok(pub_list) => pub_list,
    };

    execute_calculations(pub_list, path_dir);
}
