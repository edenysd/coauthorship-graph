mod custom_writer;
mod reader;
mod types;
mod utils;

use crate::{
    custom_writer::{
        write_co_authorship_freq, write_exclusivity_per_pub, write_normalized_weights,
        write_total_co_authorship_freq_per_author,
    },
    reader::read_publication_list,
};
use std::{fs::create_dir, path::Path, process};

fn execute_calculations(pub_list: Vec<types::SimplePublication>, path_dir: String) {
    println!("calculating exclusivity_per_pub");
    let exclusivity_per_pub = utils::calculations::calculate_exclusivity_per_pub(&pub_list);
    println!("exclusivity_per_pub finished");

    println!("calculating co_authorship_freq");
    let co_authorship_freq =
        utils::calculations::calculate_co_authorship_freq(&exclusivity_per_pub);
    println!("co_authorship_freq finished");

    println!("calculating total_co_authorship_freq_per_author");
    let total_co_authorship_freq_per_author =
        utils::calculations::calculate_total_co_authorship_freq_per_author(&co_authorship_freq);
    println!("total_co_authorship_freq_per_author finished");

    println!("calculating normalized_weights");
    let normalized_weights = utils::calculations::calculate_normalized_weights(
        &co_authorship_freq,
        &total_co_authorship_freq_per_author,
    );
    println!("normalized_weights finished");

    write_exclusivity_per_pub(exclusivity_per_pub, &path_dir);
    write_co_authorship_freq(co_authorship_freq, &path_dir);
    write_total_co_authorship_freq_per_author(total_co_authorship_freq_per_author, &path_dir);
    write_normalized_weights(normalized_weights, &path_dir);
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
