use crate::my_dependency::MyDependency;

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
    use crate::my_dependency::MockMyDependency;

    #[test]
    fn example_function_test() {
        let mut mock = MockMyDependency::new();
        mock.expect_get_value()
            .with()
            .times(1)
            .returning(|| 0);
        let my_dependency = Box::new(mock);
        let kata = Kata::init(my_dependency);

        let result = kata.example_function();

        assert_eq!(result, 0);
    }
}
