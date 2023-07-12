use crate::types::SimplePublication;
use csv::Reader;
use std::{error::Error, fs::File, vec};

use ustr::Ustr;

const CSV_PATH: &str = "input.csv";

fn create_reader() -> Result<Reader<File>, Box<dyn Error>> {
    // Build the CSV reader
    let rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_path(CSV_PATH)?;

    Ok(rdr)
}

pub fn read_publication_list() -> Result<Vec<SimplePublication>, Box<dyn Error>> {
    let mut pub_list = vec![];

    // Build the CSV reader and iterate over each record.
    let mut rdr = create_reader()?;

    for result in rdr.records().into_iter() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let line = match result {
            Err(err) => {
                println!("error parsing: {}", err);
                continue;
            }
            Ok(v) => v,
        };

        let mut record: SimplePublication =
            match line.clone().deserialize::<SimplePublication>(None) {
                Err(err) => {
                    println!("error running example: {}\n in line {:?}", err, line);
                    continue;
                }
                Ok(v) => v,
            };

        let mut vec_string = line[9].to_string();

        vec_string = vec_string.replace("\"", "\\\'");
        vec_string = vec_string.replace("'", "\"");
        vec_string = vec_string.replace("\\\"", "'");

        record.coauthors = serde_json::from_str::<Vec<Ustr>>(&vec_string)?;

        pub_list.push(record);

        // if pub_list.len() > 1000000 {
        //     break;
        // }
    }

    Ok(pub_list)
}
