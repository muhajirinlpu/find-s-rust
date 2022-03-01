use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use clap::Parser;
use console::{style, Term};
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;

#[derive(Parser)]
#[clap(name = "Find-S - Rust")]
struct Cli {
    #[clap(parse(from_os_str), value_name = "CSV FILE")]
    file_path: PathBuf,
}

struct Dataset {
    headers: Vec<String>,
    data: Vec<HashMap<String, String>>,
}

fn parse_csv(file_path: PathBuf) -> Dataset {
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

fn main() {
    let cli = Cli::parse();

    println!("using file {:?} as dataset.", cli.file_path);

    let dataset = parse_csv(cli.file_path);

    let attributes = dataset.headers[..dataset.headers.len() - 1].to_owned();
    let target = dataset.headers[dataset.headers.len() - 1].to_string();

    let mut hypotesis: HashMap<String, String> = HashMap::new();
    let mut options: HashMap<String, HashSet<String>> = HashMap::new();

    for attribute in attributes.iter() {
        hypotesis.insert(attribute.to_string(), "".to_string());
    }

    for datum in dataset.data {
        if datum.contains_key(&target) && datum[&target] == "yes".to_string() {
            for attribute in attributes.iter() {
                let value = datum.get(attribute).clone().unwrap();

                if hypotesis.get(attribute).unwrap() == "" {
                    hypotesis.insert(attribute.to_string(), value.to_string());
                } else if hypotesis.get(attribute).unwrap() != value {
                    hypotesis.insert(attribute.to_string(), "*".to_string());
                }
            }
        }
        for attribute in attributes.iter() {
            let value = datum.get(attribute).clone().unwrap();

            (*options.entry(attribute.to_string()).or_insert(HashSet::new()))
                .insert(value.to_string());
        }

    }

    println!("{} {:?}", style("> hypothesis :").black().bold(), style(hypotesis.clone()).bg(console::Color::Yellow));

    let mut test: HashMap<String, String> = HashMap::new();

    for attribute in attributes.iter() {
        let items = options.get(attribute).unwrap().iter().map(|f| f.to_string()).collect::<Vec<String>>();
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("{} :", attribute))
            .items(&items)
            .default(0)
            .interact_on_opt(&Term::stderr()).unwrap();

        let index = selection.unwrap();

        test.insert(attribute.to_string(), items[index].to_string());
    }

    let mut correct = true;

    for attribute in attributes.iter() {
        let hypothesis_value = hypotesis.get(attribute).unwrap().to_string();
        if hypothesis_value != "*".to_string() && hypothesis_value != test.get(attribute).unwrap().to_string() {
            correct = false;
        }
    }

    if correct {
        println!("{} {}", target, style("✔").green());
    } else {
        println!("{} {}", target, style("✘").red());
    }
}
