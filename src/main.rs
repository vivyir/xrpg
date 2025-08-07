use std::collections::{HashMap, HashSet};

// mermaid node shapes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum NodeShape {
    Rectangle,      // [ ]
    Rounded,        // ( )
    Circle,         // (( ))
    Rhombus,        // { }
    Hexagon,        // {{ }}
    Stadium,        // ([ ])
    Subroutine,     // [[ ]]
    Cylinder,       // [( )]
    CircleDouble,   // ((( )))
    Asymmetric,     // > ]
}

impl NodeShape {
    fn to_mermaid(self, content: &str) -> String {
        match self {
            NodeShape::Rectangle => format!("[\"{}\"]", content),
            NodeShape::Rounded => format!("(\"{}\")", content),
            NodeShape::Circle => format!("((\"{}\"))", content),
            NodeShape::Rhombus => format!("{{\"{}\"}}", content),
            NodeShape::Hexagon => format!("{{\"{}\"}}", content),
            NodeShape::Stadium => format!("([\"{}\"])", content),
            NodeShape::Subroutine => format!("[[\"{}\"]]", content),
            NodeShape::Cylinder => format!("[(\"{}\")]", content),
            NodeShape::CircleDouble => format!("(((\"{}\")))", content),
            NodeShape::Asymmetric => format!(">\"{}\"]", content),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct EdgeData {
    distance: u32,
}

#[derive(Debug)]
struct MapNode {
    name: String,
    shape: NodeShape,
    removed: bool,
}

#[derive(Debug)]
struct Map {
    nodes: Vec<MapNode>,
    node_indices: HashMap<String, usize>,
    adjacency: Vec<HashMap<usize, EdgeData>>,
}

impl Map {
    pub fn new() -> Self {
        Map {
            nodes: Vec::new(),
            node_indices: HashMap::new(),
            adjacency: Vec::new(),
        }
    }

    /// Adds a node to the map and returns its index
    pub fn add_node(&mut self, name: &str, shape: NodeShape) -> usize {
        let index = self.nodes.len();
        self.nodes.push(MapNode {
            name: name.to_string(),
            shape,
            removed: false,
        });
        self.adjacency.push(HashMap::new());
        self.node_indices.insert(name.to_string(), index);
        index
    }

    /// Gets a node's index by name, ignoring removed nodes
    pub fn get_index(&self, name: &str) -> Option<usize> {
        self.node_indices.get(name).copied()
            .filter(|&idx| !self.nodes[idx].removed)
    }

    /// Adds a path between nodes with distance
    pub fn add_path(&mut self, from: usize, to: usize, distance: u32) {
        if from >= self.adjacency.len() || to >= self.adjacency.len() {
            return;
        }
        
        self.adjacency[from].insert(to, EdgeData { distance });
    }

    /// Adds a bidirectional path between nodes
    pub fn add_bidirectional_path(&mut self, a: usize, b: usize, distance: u32) {
        self.add_path(a, b, distance);
        self.add_path(b, a, distance);
    }

    /// Checks if a directed path exists
    pub fn has_directed_path(&self, from: usize, to: usize) -> bool {
        self.adjacency.get(from)
            .and_then(|edges| edges.get(&to))
            .is_some()
    }

    /// Checks if a bidirectional path exists
    pub fn is_bidirectional(&self, a: usize, b: usize) -> bool {
        self.has_directed_path(a, b) && self.has_directed_path(b, a)
    }

    /// Removes a directed path
    pub fn remove_directed_path(&mut self, from: usize, to: usize) -> bool {
        if let Some(edges) = self.adjacency.get_mut(from) {
            edges.remove(&to).is_some()
        } else {
            false
        }
    }

    /// Removes a bidirectional path
    pub fn remove_bidirectional_path(&mut self, a: usize, b: usize) {
        self.remove_directed_path(a, b);
        self.remove_directed_path(b, a);
    }

    /// Removes a node and all its connections
    pub fn remove_node(&mut self, index: usize) {
        if index >= self.nodes.len() || self.nodes[index].removed {
            return;
        }
        
        self.nodes[index].removed = true;
        
        // remove from name index
        if let Some(name) = self.node_indices.remove(&self.nodes[index].name) {
            // clear all outgoing connections
            self.adjacency[index].clear();
            
            // remove all incoming connections
            for edges in &mut self.adjacency {
                edges.remove(&index);
            }
        }
    }

    /// Gets all neighbors of a node
    pub fn neighbors(&self, node: usize) -> Vec<usize> {
        if node >= self.adjacency.len() || self.nodes[node].removed {
            return Vec::new();
        }
        
        self.adjacency[node].keys().copied().collect()
    }

    /// Generates Mermaid.js flowchart without duplicate bidirectional paths
    pub fn to_mermaid(&self) -> String {
        let mut output = String::from("graph LR\n");
        let mut valid_nodes = HashSet::new();
        let mut rendered_edges = HashSet::new();
        
        // add active nodes with
        for (idx, node) in self.nodes.iter().enumerate() {
            if node.removed {
                continue;
            }
            
            valid_nodes.insert(idx);
            output.push_str(&format!(
                "    {}{}\n",
                idx,
                node.shape.to_mermaid(&sanitize_mermaid(&node.name))
            ));
        }
        
        // add paths with distance labels
        for (from, edges) in self.adjacency.iter().enumerate() {
            if !valid_nodes.contains(&from) {
                continue;
            }
            
            for (to, edge_data) in edges {
                if !valid_nodes.contains(to) {
                    continue;
                }
                
                // skip if we've already rendered this bidirectional pair
                let reverse_key = (*to, from);
                let key = (from, *to);
                
                if rendered_edges.contains(&key) {
                    continue;
                }
                
                if self.is_bidirectional(from, *to) {
                    // only render bidirectional edges once
                    rendered_edges.insert(key);
                    rendered_edges.insert(reverse_key);
                    
                    output.push_str(&format!(
                        "    {} <--> |{} min| {}\n",
                        from,
                        edge_data.distance,
                        to
                    ));
                } else {
                    // render unidirectional paths
                    output.push_str(&format!(
                        "    {} --> |{} min| {}\n",
                        from,
                        edge_data.distance,
                        to
                    ));
                }
            }
        }
        
        // add styling directives
        output.push_str("\n    classDef active fill:#f0f0f0,stroke:#333,stroke-width:2px;\n");
        output.push_str("    linkStyle default stroke:#777,stroke-width:2px,fill:none;\n");
        
        output
    }
}

/// Sanitizes text for Mermaid.js
fn sanitize_mermaid(text: &str) -> String {
    text.replace('"', "#quot;")
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('|', "&#124;")
}

fn main() {
    let mut map = Map::new();
    
    let town = map.add_node("Home Town", NodeShape::Circle);
    let dungeon = map.add_node("Dark Dungeon", NodeShape::Asymmetric);
    let cave = map.add_node("Dragon's Cave", NodeShape::Rhombus);
    let forest = map.add_node("Enchanted Forest", NodeShape::Stadium);
    let castle = map.add_node("Royal Castle", NodeShape::Hexagon);
    
    map.add_bidirectional_path(town, forest, 15);
    map.add_bidirectional_path(town, castle, 25);
    map.add_path(forest, dungeon, 45);  // one-way path
    map.add_bidirectional_path(dungeon, cave, 30);
    map.add_bidirectional_path(castle, dungeon, 10);
    
    println!("=== Initial Map ===");
    println!("{}", map.to_mermaid());
    
    println!(
        "Path from Town to Forest is bidirectional: {}",
        map.is_bidirectional(town, forest)
    );
    println!(
        "Path from Forest to Dungeon is bidirectional: {}",
        map.is_bidirectional(forest, dungeon)
    );
    
    map.remove_directed_path(forest, dungeon);
    println!("\n=== After removing Forest->Dungeon path ===");
    println!("{}", map.to_mermaid());
    
    map.remove_node(dungeon);
    println!("\n=== After removing Dungeon node ===");
    println!("{}", map.to_mermaid());
    
    let village = map.add_node("River Village", NodeShape::Rounded);
    map.add_bidirectional_path(town, village, 20);
    println!("\n=== After adding River Village ===");
    println!("{}", map.to_mermaid());
}
