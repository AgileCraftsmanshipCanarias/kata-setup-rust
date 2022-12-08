use crate::my_dependency::MyDependency;
use crate::my_dependency_impl::MyDependencyImpl;

pub struct Kata {
    my_dependency: Box<dyn MyDependency>,
}

impl Kata {
    // inject the dependency over the init fn
    pub fn init(my_dependency: Box<dyn MyDependency>) -> Kata {
        Kata { my_dependency }
    }

    pub fn example_function(&self) -> u32 {
        self.my_dependency.get_value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_function_test() {
        let my_dependency = Box::new(MyDependencyImpl {});
        let kata = Kata::init(my_dependency);

        assert_eq!(kata.example_function(), 0);
    }
}
