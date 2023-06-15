use std::{
    collections::HashMap,
    io::{self},
    vec,
};

fn main() {
    let mut trie_tree = TrieStruct::create();
    let mut line_vec: Vec<String> = vec![];
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.is_empty() {
            break;
        }
        trie_tree.insert(input.trim().to_string());
        line_vec.push(input.trim().to_string());
    }
    for i in line_vec {
        println!("{} {}", i, trie_tree.find_substr(i.to_string()));
    }
}

struct TireNode {
    value: Option<char>,
    is_final: bool,
    child_nodes: HashMap<char, TireNode>,
    visit_counter: u32,
}

impl TireNode {
    pub fn new(c: char, is_final: bool) -> TireNode {
        TireNode {
            value: Option::Some(c),
            is_final,
            child_nodes: HashMap::new(),
            visit_counter: 1,
        }
    }

    pub fn new_root() -> TireNode {
        TireNode {
            value: Option::None,
            is_final: false,
            child_nodes: HashMap::new(),
            visit_counter: u32::MAX,
        }
    }

    pub fn check_value(self, c: char) -> bool {
        self.value == Some(c)
    }

    pub fn insert_value(&mut self, c: char, is_final: bool) {
        self.child_nodes.insert(c, TireNode::new(c, is_final));
    }
}

struct TrieStruct {
    root_node: TireNode,
}

impl TrieStruct {
    pub fn create() -> TrieStruct {
        TrieStruct {
            root_node: TireNode::new_root(),
        }
    }

    pub fn insert(&mut self, string_val: String) {
        let mut current_node = &mut self.root_node;
        let char_vec: Vec<char> = string_val.chars().collect();
        let mut last_match = 0;

        for i in 0..char_vec.len() {
            if current_node.child_nodes.contains_key(&char_vec[i]) {
                current_node = current_node.child_nodes.get_mut(&char_vec[i]).unwrap();
                current_node.visit_counter += 1;
            } else {
                last_match = i;
                break;
            }
            last_match = i + 1;
        }

        if last_match == char_vec.len() {
            current_node.is_final = true;
        } else {
            for i in last_match..char_vec.len() {
                current_node.insert_value(char_vec[i], false);
                current_node = current_node.child_nodes.get_mut(&char_vec[i]).unwrap();
            }
            current_node.is_final = true;
        }
    }

    pub fn find_substr(&mut self, string_val: String) -> String {
        let mut current_node = &mut self.root_node;
        let char_vec: Vec<char> = string_val.chars().collect();

        for i in 0..char_vec.len() {
            current_node = current_node.child_nodes.get_mut(&char_vec[i]).unwrap();
            if current_node.visit_counter == 1 {
                return string_val[0..=i].to_string();
            }
        }

        string_val
    }
}
