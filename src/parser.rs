use std::collections::HashMap;
use std::path::PathBuf;

pub struct Dataset {
    pub(crate) headers: Vec<String>,
    pub(crate) data: Vec<HashMap<String, String>>,
}

pub fn parse_csv(file_path: PathBuf) -> Dataset {
    let mut rdr = csv::Reader::from_path(file_path).expect("Failed to read file");
    let headers = rdr.headers().expect("Failed to read headers").clone();

    let data_headers = headers.iter().map(|h| h.to_string()).collect();
    let mut data: Vec<HashMap<String, String>> = vec![];

    for result in rdr.records() {
        let record = result.expect("Failed to parse record");

        let mut map = HashMap::new();
        for (i, field) in record.iter().enumerate() {
            map.insert(headers[i].to_string(), field.to_string());
        }

        data.push(map);
    }

    Dataset {
        headers: data_headers,
        data,
    }
}
