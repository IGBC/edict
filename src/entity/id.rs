use core::fmt;

/// Entity ID.
/// Does not contain generation.
/// IDs may change what entity they refer to between `World::maintenance` calls.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct EntityId {
    pub id: u32,
}

impl fmt::Debug for EntityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EntityId").field("id", &self.id).finish()
    }
}

impl fmt::Display for EntityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{:X}}}", self.id)
    }
}
