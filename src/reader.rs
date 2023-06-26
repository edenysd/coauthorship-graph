use crate::types::Publication;
use csv::Reader;
use std::{error::Error, fs::File, usize, vec};

const CSV_PATH: &str = "input.csv";

fn create_reader() -> Result<Reader<File>, Box<dyn Error>> {
    // Build the CSV reader
    let rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_path(CSV_PATH)?;
    Ok(rdr)
}

pub fn read_publication_list(limit: usize) -> Result<Vec<Publication>, Box<dyn Error>> {
    let mut pub_list = vec![];

    // Build the CSV reader and iterate over each record.
    let mut rdr = create_reader()?;

    for result in rdr.records().into_iter() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let line = result?;

        let mut record: Publication = match line.clone().deserialize::<Publication>(None) {
            Err(err) => {
                println!("error running example: {}\n in line {:?}", err, line);
                continue;
            }
            Ok(v) => v,
        };

        let mut vec_string = line[9].to_string();
        vec_string = vec_string.replace("'", "\"");
        vec_string = vec_string.replace("\\\"", "'");

        record.coauthors = serde_json::from_str::<Vec<String>>(&vec_string)?;

        if pub_list.len() >= limit {
            break;
        }

        pub_list.push(record);
    }

    Ok(pub_list)
}
