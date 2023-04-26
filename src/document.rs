use std::fs;
use std::path::Path;
use std::str::FromStr;

use toml_edit::{Array, Formatted, InlineTable, Item, Key, Value};

use crate::crates::Crate;
use crate::index::Index;

pub struct Document {
    toml_doc: toml_edit::Document,
    index: Index,

    crates: Vec<Crate>,

    path: String,
}

impl Document {
    pub fn new<P: AsRef<Path>>(path: P, index: Index) -> anyhow::Result<Document> {
        let file_content = fs::read_to_string(&path).unwrap();
        let doc = toml_edit::Document::from_str(&file_content).unwrap();

        let (_name, deps) = match doc.get_key_value("dependencies") {
            None => {
                return Err(anyhow::Error::msg("no dependencies were found"))
            }
            Some(some) => {some}
        };

        let deps = deps.as_table().unwrap();

        let mut crates = vec![];

        for (name, value) in deps {
            crates.push(index.get_crate(name, value).unwrap());
        }

        Ok(Document {
            toml_doc: doc,
            index,
            crates,
            path: path.as_ref().to_str().unwrap().to_string(),
        })
    }

    pub fn get_deps(&self) -> &Vec<Crate> {
        &self.crates
    }

    pub fn get_dep(&self, index: usize) -> anyhow::Result<&Crate> {
        match self.crates.get(index) {
            None => Err(anyhow::Error::msg("out of bounce")),
            Some(some) => Ok(some),
        }
    }

    pub fn get_deps_mut(&mut self) -> &mut Vec<Crate> {
        &mut self.crates
    }

    pub fn write_dep(&mut self, dep_index: usize) {
        let (_name, deps) = self.toml_doc.get_key_value_mut("dependencies").unwrap();
        let deps = deps.as_table_mut().unwrap();

        let current_crate = self.crates.get(dep_index).unwrap();

        if !current_crate.uses_default() || current_crate.get_enabled_features().len() != 0 {
            let mut table = InlineTable::new();

            //version
            table.insert(
                "version",
                Value::String(Formatted::new(current_crate.get_version())),
            );

            //features
            let mut features = Array::new();

            for name in current_crate.get_enabled_features() {
                features.push(Value::String(Formatted::new(name)));
            }

            table.insert("features", Value::Array(features));

            //default-feature
            let uses_default = current_crate.uses_default();
            if !uses_default {
                table.insert(
                    "default-features",
                    Value::Boolean(Formatted::new(uses_default)),
                );
            }

            deps.insert(
                &current_crate.get_name(),
                Item::Value(Value::InlineTable(table)),
            );
        } else {
            deps.insert(
                &current_crate.get_name(),
                Item::Value(Value::String(Formatted::new(current_crate.get_version()))),
            );
        }

        fs::write(self.path.clone(), self.toml_doc.to_string()).unwrap();
    }
}
