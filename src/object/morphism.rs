use super::object::ObjectTrait;

pub struct morphism<'a> {
    initial_object: &'a dyn ObjectTrait,
    final_object: &'a dyn ObjectTrait
}
