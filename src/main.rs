mod reader;
mod types;
mod utils;
mod writer;

use crate::reader::read_publication_list;
use std::{fs::File, process};

fn main() {
    let pub_list = match read_publication_list(1000000) {
        Err(err) => {
            println!("error running example: {}", err);
            process::exit(1);
        }
        Ok(pub_list) => pub_list,
    };

    let exclusivity_per_pub = utils::calculations::calculate_exclusivity_per_pub(&pub_list);

    let co_authorship_freq =
        utils::calculations::calculate_co_authorship_freq(&exclusivity_per_pub);

    let total_co_authorship_freq_per_author =
        utils::calculations::calculate_total_co_authorship_freq_per_author(&co_authorship_freq);

    let normalized_weights = utils::calculations::calculate_normalized_weights(
        &co_authorship_freq,
        &total_co_authorship_freq_per_author,
    );
}
