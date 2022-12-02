use std::collections::HashSet;



#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub children: HashSet<String>
}

impl Node {
    pub fn is_big(name: &String) -> bool {
        return name.chars().nth(0).unwrap().is_uppercase();
    }

    pub fn is_small(name: &String) -> bool {
        return !Node::is_big(name);
    }
}