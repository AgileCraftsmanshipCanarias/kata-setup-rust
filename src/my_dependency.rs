#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait MyDependency {
    fn get_value(&self) -> u32;
}
