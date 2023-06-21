use std::{error::Error, process, usize, vec};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Publication{
    block_fullname: String,
    author_group_orcid: String,
    author_group_idx_in_block: String,
    citation_idx_in_author_group: String,
    doi: String,
    pid: String,
    author_position: u64,
    author_name: String,
    author_affiliation: String,
    #[serde(skip_deserializing)]
    coauthors: Vec<String>,
    coauthor_affliations: String,
    venue: String,
    pub_year: String,
    paper_title: String,
    paper_abstract: String
}

fn read_publication_list( limit: usize) -> Result< Vec<Publication>, Box<dyn Error>> {
    let mut pub_list = vec![];
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::ReaderBuilder::new()
                .has_headers(true)
                .flexible(true)
                .from_path("input.csv")?;
    
    for result in rdr.records().into_iter() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let line = result?;
        
        let mut record:Publication = match line.clone().deserialize::<Publication>(None){
            Err(err)=>{
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


fn main() {
    println!("{:?}",serde_json::from_str::<Vec<String>>(r#"["asd","asd"]"#));
    match read_publication_list(1000000) {
        Err(err) => {
            println!("error running example: {}", err);
            process::exit(1);
        }
        Ok(pub_list) => {
            for publication in pub_list.into_iter(){
                println!("{:?}", publication.coauthors);
            }
        }
    }

    
}
