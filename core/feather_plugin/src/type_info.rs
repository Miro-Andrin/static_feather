use std::any::{type_name, TypeId};

#[derive(Eq, Clone)]
pub struct TypeInfo {
    /// Type name
    pub name: &'static str,
    pub id: TypeId,
}

impl PartialEq for TypeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl std::hash::Hash for TypeInfo {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl TypeInfo {
    pub fn of<A: 'static>() -> TypeInfo {
        TypeInfo {
            name: type_name::<A>(),
            id: TypeId::of::<A>(),
        }
    }
}
