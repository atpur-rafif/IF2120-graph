pub struct Graph {
    pub node: Vec<String>,
    pub edge: Vec<(String, String)>,
}

impl Graph {
    pub fn from_file(node_path: &str, edge_path: &str) -> Graph {
        use std::fs;

        let node: Vec<String> = match fs::read_to_string(node_path) {
            Ok(str) => {
                let node: Vec<String> = str.split('\n').map(|s| s.trim().to_string()).collect();
                if !node.iter().all(|s| !s.contains(' ')) {
                    panic!("Node can't include space");
                }
                node
            }
            Err(e) => panic!("{}", e),
        };

        let edge: Vec<(String, String)> = match fs::read_to_string(edge_path) {
            Ok(str) => str
                .split('\n')
                .map(|pair| {
                    let edge: Vec<&str> = pair.trim().split(' ').collect();
                    if edge.len() != 2 {
                        panic!("Invalid edge format at \"{pair}\"");
                    }
                    (edge[0].to_string(), edge[1].to_string())
                })
                .collect(),
            Err(e) => panic!("{}", e),
        };

        Graph { node, edge }
    }
}
