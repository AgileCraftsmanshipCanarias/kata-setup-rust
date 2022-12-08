use crate::my_dependency_impl::MyDependencyImpl;

mod kata;
mod my_dependency;
mod my_dependency_impl;

fn main() {
    let my_dependency = Box::new(MyDependencyImpl {});
    let kata = kata::Kata::init(my_dependency);

    let value = kata.example_function();

    println!("The value is {}", value);
}
