use crate::my_dependency::MyDependency;

pub struct MyDependencyImpl {}

impl MyDependency for MyDependencyImpl {
    fn get_value(&self) -> u32 {
        0
    }
}
