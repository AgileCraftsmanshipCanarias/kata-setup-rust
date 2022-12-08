use mockall::automock;

#[automock]
pub trait MyDependency {
    fn get_value(&self) -> u32;
}
