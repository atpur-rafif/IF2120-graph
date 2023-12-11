use std::collections::HashMap;
use std::mem::swap;

use crate::graph::Graph;

#[derive(Debug)]
pub struct DSU {
    data: HashMap<String, String>,
}

impl DSU {
    pub fn find(&mut self, key: &String) -> String {
        let value = self
            .data
            .get(key)
            .expect(format!("Invalid node for DSU {key}").as_str())
            .clone();

        if value == *key {
            value
        } else {
            let value = self.find(&value);
            self.data
                .entry(key.to_string())
                .and_modify(|v| *v = value.clone());
            value
        }
    }

    pub fn join(&mut self, a: &String, b: &String) {
        let mut a = self.find(a);
        let mut b = self.find(b);

        if b < a {
            swap(&mut a, &mut b);
        }

        self.data.entry(b).and_modify(|v| *v = a);
    }

    pub fn insert(&mut self, a: &String) {
        self.data.insert(a.clone(), a.clone());
    }

    pub fn from_graph(graph: &Graph) -> DSU {
        let mut dsu = DSU {
            data: HashMap::new(),
        };

        for node in &graph.node {
            dsu.insert(node);
        }

        for (a, b) in &graph.edge {
            dsu.join(a, b);
        }

        for node in &graph.node {
            dsu.find(node);
        }

        dsu
    }

    pub fn group(&self) -> Vec<Vec<String>> {
        let mut hash: HashMap<String, Vec<String>> = HashMap::new();

        for (k, v) in &self.data {
            let entry = hash.entry(v.clone()).or_insert(Vec::new());
            entry.push(k.clone());
        }

        let mut res: Vec<Vec<String>> = Vec::new();
        for (_, v) in hash {
            res.push(v);
        }

        res
    }
}
