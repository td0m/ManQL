use crate::ast::{Scope, Set, Snippet};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::{io::BufWriter, path::Path};

use std::cmp::Ordering;

// TODO: replace Vec with actual Set
#[derive(Serialize, Deserialize, Default)]
pub struct Cache {
    #[serde(skip)]
    file: String,
    // each set has an array of most recently used values (from oldest to most recent)
    sets: HashMap<String, SetCache>,
    // array containing most recently used snippets (from oldest to most recent)
    snippet_history: Vec<String>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
struct SetCache {
    pub history: Vec<String>,
    pub custom_values: Vec<String>,
}

impl Cache {
    pub fn new(file_path: &str) -> Self {
        let mut cache = Cache {
            file: String::from(file_path),
            ..Default::default()
        };
        let path = Path::new(file_path);
        if !path.exists() {
            cache.save();
        }
        let file_content = read_to_string(file_path);
        if let Ok(content) = file_content {
            cache = serde_json::from_str(&content).unwrap();
            cache.file = String::from(file_path);
        }
        cache
    }

    pub fn save(&self) {
        let file = File::create(&self.file).expect("cannot create file");
        let w = BufWriter::new(file);
        serde_json::to_writer_pretty(w, &self).unwrap();
    }

    pub fn push_snippet(&mut self, snippet: &Snippet) {
        self.snippet_history = self
            .snippet_history
            .clone()
            .into_iter()
            .filter(|s| String::from(s) != snippet.description)
            .collect::<Vec<String>>();
        self.snippet_history
            .push(String::from(&snippet.description));
    }

    pub fn push_set_value(&mut self, set: &Set, value: &str) {
        let mut s = self.get_or_create_set(set);
        s.history = s
            .history
            .clone()
            .into_iter()
            .filter(|s| String::from(s) != value)
            .collect::<Vec<String>>();
        s.history.push(String::from(value));
    }

    // assumes that the custom value doesn't already exist
    pub fn push_custom_value(&mut self, set: &Set, value: &str) {
        self.get_or_create_set(set)
            .custom_values
            .push(String::from(value));
    }

    pub fn snippet_partial_cmp(&self, s1: &Snippet, s2: &Snippet) -> Ordering {
        self.get_snippet_position(s2)
            .cmp(&self.get_snippet_position(s1))
    }
    pub fn value_partial_cmp(&mut self, set: &Set, s1: &str, s2: &str) -> Ordering {
        self.get_value_position(set, s2)
            .cmp(&self.get_value_position(set, s1))
    }

    // returns position of snippet in history. the lower the more recently used.
    // if not found returns the max usize value
    pub fn get_snippet_position(&self, snippet: &Snippet) -> usize {
        for (i, description) in self.snippet_history.iter().enumerate() {
            if String::from(description) == snippet.description {
                return i + 1;
            }
        }
        return 0; // TODO: return max
    }
    // returns position of a value in set history. the lower the more recently used.
    // if not found returns the max usize value
    pub fn get_value_position(&mut self, set: &Set, value: &str) -> usize {
        let set = self.get_or_create_set(set);
        for (i, v) in set.history.iter().enumerate() {
            if v == value {
                return i + 1;
            }
        }

        return 0;
    }

    pub fn get_custom_values(&self, set: &Set) -> Vec<String> {
        if let Some(set_cache) = self.sets.get(&set.identifier) {
            return set_cache.custom_values.clone();
        }
        vec!["fak".to_owned()]
    }

    fn get_or_create_set(&mut self, set: &Set) -> &mut SetCache {
        if !self.sets.contains_key(&set.identifier) {
            self.sets.insert(
                String::from(&set.identifier),
                SetCache {
                    ..Default::default()
                },
            );
        }
        return self.sets.get_mut(&set.identifier).unwrap();
    }
}
