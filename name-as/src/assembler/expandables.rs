use std::fmt;

pub(crate) trait Expandable: fmt::Debug {
    fn expand(&self, input: &str) -> String;
}

#[derive(Debug)]
pub(crate) struct Equivalence {
    pub(crate) name: String,
    pub(crate) expansion: String,
}

impl Expandable for Equivalence {
    fn expand(&self, input: &str) -> String {
        let mut result = String::new();
        let mut start = 0;
        
        while let Some(pos) = input[start..].find(&self.name) {
            result.push_str(&input[start..start+pos]);
            result.push_str(&self.expansion);
            start += pos + self.name.len();
        }

        result.push_str(&input[start..]);
        result
    }
}