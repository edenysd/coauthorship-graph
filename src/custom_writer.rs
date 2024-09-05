use serde::Serialize;
use std::{fs::File, io::BufWriter};

use ustr::UstrMap;

pub fn write_co_authorship_freq(co_authorship_freq: &UstrMap<UstrMap<f32>>, path_dir: &String) {
    println!("Init writer for co_authorship_freq");
    serde_json::to_writer(
        BufWriter::with_capacity(
            1000000,
            File::create(path_dir.clone() + "co_authorship_freq.json")
                .expect("Failed to create file."),
        ),
        &co_authorship_freq,
    )
    .expect("Failed to write file");

    println!("Finish writer for co_authorship_freq");
}

pub fn write_total_co_authorship_freq_per_author(
    total_co_authorship_freq_per_author: &UstrMap<f32>,
    path_dir: &String,
) {
    println!("Init writer for total_co_authorship_freq_per_author");
    serde_json::to_writer(
        BufWriter::with_capacity(
            1000000,
            File::create(path_dir.clone() + "total_co_authorship_freq_per_author.json")
                .expect("Failed to create file."),
        ),
        &total_co_authorship_freq_per_author,
    )
    .expect("Failed to write file");

    println!("Finish writer for total_co_authorship_freq_per_author");
}

pub fn write_normalized_weights(normalized_weights: &UstrMap<UstrMap<f32>>, path_dir: &String) {
    println!("Init writer for normalized_weights");
    serde_json::to_writer(
        BufWriter::with_capacity(
            1000000,
            File::create(path_dir.clone() + "normalized_weights.json")
                .expect("Failed to create file."),
        ),
        &normalized_weights,
    )
    .expect("Failed to write file");

    println!("Finish writer for normalized_weights");
}
