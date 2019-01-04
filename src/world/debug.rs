use std::fmt;

use super::{Entity, World};

pub struct EntityDebug<'a>(pub &'a World, pub Entity);

impl<'a> fmt::Debug for EntityDebug<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[cfg(feature = "nightly")]
        let r = self.0.write_entity_debug(self.1, f);
        #[cfg(not(feature = "nightly"))]
        let r = f
            .debug_struct("Entity").field("entity", &self.1)
            .field("note", &"The `nightly` feature of Specs is disabled, \
            which makes this representation not very useful.")
            .finish();

        r
    }
}

pub struct EntitiesDebug<'a>(pub &'a World, pub Vec<Entity>);

impl<'a> fmt::Debug for EntitiesDebug<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut debug = f.debug_list();

        for &entity in &self.1 {
            debug.entry(&EntityDebug(self.0, entity));
        }

        debug.finish()
    }
}
