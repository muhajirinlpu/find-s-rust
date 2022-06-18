use std::collections::{HashMap, HashSet};
use console::{style, Term};
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;

use crate::methods::concern::predicable::{Predicable, Prediction};
use crate::methods::concern::trainable::Trainable;
use crate::parser::Dataset;

pub struct FindS {
    dataset: Dataset,
    hypothesis: HashMap<String, String>,
    options: HashMap<String, HashSet<String>>,
    attributes: Vec<String>,
    possitive_value: String,
}

impl FindS {
    pub(crate) fn new(dataset: Dataset) -> Self {
        FindS {
            dataset,
            hypothesis: HashMap::new(),
            options: HashMap::new(),
            attributes: Default::default(),
            possitive_value: "".to_string(),
        }
    }
}

impl Trainable for FindS {
    fn train(&mut self) {
        self.attributes = self.dataset.headers[..self.dataset.headers.len() - 1].to_owned();
        let target = self.dataset.headers[self.dataset.headers.len() - 1].to_string();

        // init hypothesis
        for attribute in self.attributes.iter() {
            self.hypothesis.insert(attribute.to_string(), "".to_string());
        }

        for datum in &self.dataset.data {
            if datum.contains_key(&target) && datum[&target] == "yes".to_string() {
                for attribute in self.attributes.iter() {
                    let value = datum.get(attribute).clone().unwrap();

                    if self.hypothesis.get(attribute).unwrap() == "" {
                        self.hypothesis.insert(attribute.to_string(), value.to_string());
                    } else if self.hypothesis.get(attribute).unwrap() != value {
                        self.hypothesis.insert(attribute.to_string(), "*".to_string());
                    }
                }
            }
            for attribute in self.attributes.iter() {
                let value = datum.get(attribute).clone().unwrap();

                (*self.options.entry(attribute.to_string())
                    .or_insert(HashSet::new()))
                    .insert(value.to_string());
            }
        }
    }
}

impl Predicable<String> for FindS {
    fn predict(&self) -> Vec<Prediction<String>> {
        println!("{} {:?}", style("> hypothesis :").black().bold(), style(self.hypothesis.clone()).bg(console::Color::Yellow));
        println!("{} {:?}", style("> options :").black().bold(), style(self.options.clone()).bg(console::Color::Yellow));

        let mut test: HashMap<String, String> = HashMap::new();

        for attribute in self.attributes.iter() {
            let items = self.options.get(attribute).unwrap().iter().map(|f| f.to_string()).collect::<Vec<String>>();
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt(format!("{} :", attribute))
                .items(&items)
                .default(0)
                .interact_on_opt(&Term::stderr()).unwrap();

            let index = selection.unwrap();

            test.insert(attribute.to_string(), items[index].to_string());
        }

        let mut correct = true;

        for attribute in self.attributes.iter() {
            let hypothesis_value = self.hypothesis.get(attribute).unwrap().to_string();
            if hypothesis_value != "*".to_string() && hypothesis_value != test.get(attribute).unwrap().to_string() {
                correct = false;
            }
        }

        let result: Vec<Prediction<String>> = vec![
            Prediction {
                value: "yes".to_string(),
                confidence: if correct {
                    1.0
                } else {
                    0.0
                },
            },
            Prediction {
                value: "no".to_string(),
                confidence: if !correct {
                    1.0
                } else {
                    0.0
                },
            }
        ];

        return result
    }
}
