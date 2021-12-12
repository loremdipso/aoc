use std::collections::HashSet;



#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub children: HashSet<String>
}

impl Node {
    pub fn isBig(&self) -> bool {
        return self.name.chars().nth(0).unwrap().is_uppercase();
    }
}