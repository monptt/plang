use super::object::ObjectTrait;

pub struct Morphism<'a> {
    initial_object: &'a dyn ObjectTrait,
    final_object: &'a dyn ObjectTrait
}
