use std::thread::LocalKey;

use super::storage_api::{IdentifierRefMethods, PrincipalIdentifier, StorageMethods, StorageRef};
use crate::models::{
    api_error::ApiError,
    group::Group,
    identifier::{Identifier, IdentifierKind},
};

pub struct GroupStore<'a> {
    store: &'a LocalKey<StorageRef<u64, Group>>,
    identifier_ref: &'a LocalKey<StorageRef<PrincipalIdentifier, u64>>,
}

impl<'a> GroupStore<'a> {
    pub fn new(
        store: &'a LocalKey<StorageRef<u64, Group>>,
        identifier_ref: &'a LocalKey<StorageRef<PrincipalIdentifier, u64>>,
    ) -> Self {
        Self {
            store,
            identifier_ref,
        }
    }
}

pub const NAME: &str = "groups";

impl IdentifierRefMethods<u64> for GroupStore<'static> {
    /// get a new identifier
    /// # Returns
    /// * `PrincipalIdentifier` - The new identifier
    fn new_identifier(&self) -> PrincipalIdentifier {
        let id = self.identifier_ref.with(|data| {
            data.borrow()
                .last_key_value()
                .map(|(k, _)| Identifier::from(k).id() + 1)
                .unwrap_or(0)
        });

        Identifier::generate(IdentifierKind::Profile(id))
            .to_principal()
            .unwrap()
    }

    /// Get the key by identifier
    /// # Arguments
    /// * `key` - The identifier to get the key for
    /// # Returns
    /// * `Option<u64>` - The key if found, otherwise None
    fn get_id_by_identifier(&self, key: &PrincipalIdentifier) -> Option<u64> {
        self.identifier_ref.with(|data| data.borrow().get(key))
    }

    /// Get the identifier by key
    /// # Arguments
    /// * `value` - The value to get the identifier for
    /// # Returns
    /// * `Option<PrincipalIdentifier>` - The identifier if found, otherwise None
    fn get_identifier_by_id(&self, value: &u64) -> Option<PrincipalIdentifier> {
        self.identifier_ref.with(|data| {
            data.borrow()
                .iter()
                .find(|(_, v)| v == value)
                .map(|(k, _)| k.clone())
        })
    }

    /// Insert an identifier reference
    /// # Arguments
    /// * `value` - The increment value to insert
    /// # Returns
    /// * `Result<u64, ApiError>` - The inserted u64 if successful, otherwise an error
    fn insert_identifier_ref(&mut self, value: u64) -> Result<u64, ApiError> {
        let identifier_principal = Identifier::generate(IdentifierKind::Group(value))
            .to_principal()
            .unwrap();
        self.identifier_ref.with(|data| {
            if data.borrow().contains_key(&identifier_principal) {
                return Err(ApiError::duplicate()
                    .add_method_name("insert_identifier_ref")
                    .add_info(NAME)
                    .add_message("Key already exists"));
            }

            data.borrow_mut().insert(identifier_principal, value);
            Ok(value)
        })
    }

    /// Remove an identifier reference
    /// # Arguments
    /// * `key` - The identifier to remove
    /// # Returns
    /// * `bool` - True if the identifier was removed, otherwise false
    fn remove_identifier_ref(&mut self, key: &PrincipalIdentifier) -> bool {
        self.identifier_ref
            .with(|data| data.borrow_mut().remove(key).is_some())
    }
}

impl StorageMethods<u64, Group> for GroupStore<'static> {
    /// Get a single group by key
    /// # Arguments
    /// * `key` - The key of the group to get
    /// # Returns
    /// * `Result<Group, ApiError>` - The group if found, otherwise an error
    fn get(&self, key: u64) -> Result<(u64, Group), ApiError> {
        self.store.with(|data| {
            data.borrow()
                .get(&key)
                .ok_or(ApiError::not_found().add_method_name("get").add_info(NAME))
                .map(|value| (key, value))
        })
    }

    /// Get multiple groups by key
    /// # Arguments
    /// * `ids` - The keys of the groups to get
    /// # Returns
    /// * `Vec<Group>` - The groups if found, otherwise an empty vector
    fn get_many(&self, keys: Vec<u64>) -> Vec<(u64, Group)> {
        self.store.with(|data| {
            let mut groups = Vec::new();
            for key in keys {
                if let Some(group) = data.borrow().get(&key) {
                    groups.push((key, group));
                }
            }
            groups
        })
    }

    /// Find a single group by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Option<(u64, Group)>` - The group if found, otherwise None
    fn find<F>(&self, filter: F) -> Option<(u64, Group)>
    where
        F: Fn(&u64, &Group) -> bool,
    {
        self.store
            .with(|data| data.borrow().iter().find(|(id, value)| filter(id, value)))
    }

    /// Find all groups by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Vec<(u64, Group)>` - The groups if found, otherwise an empty vector
    fn filter<F>(&self, filter: F) -> Vec<(u64, Group)>
    where
        F: Fn(&u64, &Group) -> bool,
    {
        self.store.with(|data| {
            data.borrow()
                .iter()
                .filter(|(id, value)| filter(id, value))
                .collect()
        })
    }

    /// Insert a single group
    /// # Arguments
    /// * `value` - The group to insert
    /// # Returns
    /// * `Result<Group, ApiError>` - The inserted group if successful, otherwise an error
    /// # Note
    /// Does check if a group with the same key already exists, if so returns an error
    fn insert(&mut self, value: Group) -> Result<(u64, Group), ApiError> {
        self.store.with(|data| {
            let key = data
                .borrow()
                .last_key_value()
                .map(|(k, _)| k + 1)
                .unwrap_or(0);

            if data.borrow().contains_key(&key) {
                return Err(ApiError::duplicate()
                    .add_method_name("insert")
                    .add_info(NAME)
                    .add_message("Key already exists"));
            }

            data.borrow_mut().insert(key, value.clone());
            Ok((key, value))
        })
    }

    /// This method is not supported for this storage
    /// # Note
    /// This method is not supported for this storage because the key is supplied by the canister
    /// use `insert` instead
    fn insert_by_key(&mut self, _key: u64, _value: Group) -> Result<(u64, Group), ApiError> {
        Err(ApiError::unsupported()
            .add_method_name("insert_by_key") // value should be `insert` as a string value
            .add_info(NAME)
            .add_message("This value does not require a key to be inserted, use `insert` instead"))
    }

    /// Update a single group by key
    /// # Arguments
    /// * `key` - The key of the group to update
    /// * `value` - The group to update
    /// # Returns
    /// * `Result<Group, ApiError>` - The updated group if successful, otherwise an error
    /// # Note
    /// Does check if a group with the same key already exists, if not returns an error
    fn update(&mut self, key: u64, value: Group) -> Result<(u64, Group), ApiError> {
        self.store.with(|data| {
            if !data.borrow().contains_key(&key) {
                return Err(ApiError::not_found()
                    .add_method_name("update")
                    .add_info(NAME)
                    .add_message("Key does not exist"));
            }

            data.borrow_mut().insert(key, value.clone());
            Ok((key, value))
        })
    }

    /// Remove a single group by key
    /// # Arguments
    /// * `key` - The key of the group to remove
    /// # Returns
    /// * `bool` - True if the group was removed, otherwise false
    /// # Note
    /// TODO: Check if we want to do a soft delete
    fn remove(&mut self, key: u64) -> bool {
        self.store
            .with(|data| data.borrow_mut().remove(&key).is_some())
    }
}
