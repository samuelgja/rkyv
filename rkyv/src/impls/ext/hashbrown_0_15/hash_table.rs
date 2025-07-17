use hashbrown::HashTable;

use crate::{
    collections::swiss_table::{ArchivedHashTable, HashTableResolver},
    Archive, Place,
};

impl<T> Archive for HashTable<T>
where
    T: Archive,
{
    type Archived = ArchivedHashTable<T::Archived>;
    type Resolver = HashTableResolver;

    fn resolve(&self, resolver: Self::Resolver, out: Place<Self::Archived>) {
        ArchivedHashTable::resolve_from_len(self.len(), (7, 8), resolver, out);
    }
}
