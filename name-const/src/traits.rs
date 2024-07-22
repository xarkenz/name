use std::fmt;

pub trait Expandable: fmt::Debug {
    fn expand(&self) -> String;
}