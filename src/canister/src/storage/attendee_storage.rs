use std::thread::LocalKey;

use candid::Principal;
use ic_cdk::caller;

use super::storage_api::{IdentifierRefMethods, PrincipalIdentifier, StorageMethods, StorageRef};
use canister_types::models::{
    api_error::ApiError,
    attendee::Attendee,
    identifier::{Identifier, IdentifierKind},
};

pub struct AttendeeStore<'a> {
    store: &'a LocalKey<StorageRef<Principal, Attendee>>,
    identifier_ref: &'a LocalKey<StorageRef<PrincipalIdentifier, Principal>>,
}

impl<'a> AttendeeStore<'a> {
    pub fn new(
        store: &'a LocalKey<StorageRef<Principal, Attendee>>,
        identifier_ref: &'a LocalKey<StorageRef<PrincipalIdentifier, Principal>>,
    ) -> Self {
        Self {
            store,
            identifier_ref,
        }
    }
}

pub const NAME: &str = "attendees";

impl IdentifierRefMethods<PrincipalIdentifier> for AttendeeStore<'static> {
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
    /// * `Option<Principal>` - The key if found, otherwise None
    fn get_id_by_identifier(&self, key: &PrincipalIdentifier) -> Option<Principal> {
        self.identifier_ref.with(|data| data.borrow().get(key))
    }

    /// Get the identifier by key
    /// # Arguments
    /// * `value` - The value to get the identifier for
    /// # Returns
    /// * `Option<PrincipalIdentifier>` - The identifier if found, otherwise None
    fn get_identifier_by_id(&self, value: &Principal) -> Option<PrincipalIdentifier> {
        self.identifier_ref.with(|data| {
            data.borrow()
                .iter()
                .find(|(_, v)| v == value)
                .map(|(k, _)| k.clone())
        })
    }

    /// Insert an identifier reference with the caller as value
    /// # Arguments
    /// * `key` - The increment value to insert
    /// # Returns
    /// * `Result<Principal, ApiError>` - The inserted principal if successful, otherwise an error
    fn insert_identifier_ref(&mut self, key: PrincipalIdentifier) -> Result<Principal, ApiError> {
        self.identifier_ref.with(|data| {
            if data.borrow().contains_key(&key) {
                return Err(ApiError::duplicate()
                    .add_method_name("insert_identifier_ref")
                    .add_info(NAME)
                    .add_message("Key already exists"));
            }

            data.borrow_mut().insert(key, caller());
            Ok(caller())
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

impl StorageMethods<Principal, Attendee> for AttendeeStore<'static> {
    /// Get a single attendee by key
    /// # Arguments
    /// * `key` - The user principal as key of the attendee to get
    /// # Returns
    /// * `Result<Attendee, ApiError>` - The attendee if found, otherwise an error
    fn get(&self, key: Principal) -> Result<(Principal, Attendee), ApiError> {
        self.store.with(|data| {
            data.borrow()
                .get(&key)
                .ok_or(ApiError::not_found().add_method_name("get").add_info(NAME))
                .map(|value| (key, value))
        })
    }

    /// Get multiple attendees by key
    /// # Arguments
    /// * `ids` - The keys of the attendees to get
    /// # Returns
    /// * `Vec<Attendee>` - The reports if found, otherwise an empty vector
    fn get_many(&self, keys: Vec<Principal>) -> Vec<(Principal, Attendee)> {
        self.store.with(|data| {
            let mut attendees = Vec::new();
            for key in keys {
                if let Some(attendee) = data.borrow().get(&key) {
                    attendees.push((key, attendee));
                }
            }
            attendees
        })
    }

    /// Find a single attendee by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Option<(Principal, Attendee)>` - The attendee if found, otherwise None
    fn find<F>(&self, filter: F) -> Option<(Principal, Attendee)>
    where
        F: Fn(&Principal, &Attendee) -> bool,
    {
        self.store.with(|data| {
            data.borrow()
                .iter()
                .find(|(id, value)| filter(id, value))
                .map(|(key, value)| (key, value))
        })
    }

    /// Find all attendees by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Vec<(Principal, Attendee)>` - The attendees if found, otherwise an empty vector
    fn filter<F>(&self, filter: F) -> Vec<(Principal, Attendee)>
    where
        F: Fn(&Principal, &Attendee) -> bool,
    {
        self.store.with(|data| {
            data.borrow()
                .iter()
                .filter(|(id, value)| filter(id, value))
                .map(|(key, value)| (key, value))
                .collect()
        })
    }

    /// This method is not supported for this storage
    /// # Note
    /// This method is not supported for this storage because the key is a `Principal`
    /// use `insert_by_key` instead
    fn insert(&mut self, _value: Attendee) -> Result<(Principal, Attendee), ApiError> {
        Err(ApiError::unsupported()
            .add_method_name("insert") // value should be `insert` as a string value
            .add_info(NAME)
            .add_message("This value requires a key to be inserted, use `insert_by_key` instead"))
    }

    /// Insert a single attendee by key
    /// # Arguments
    /// * `key` - The user principal as key of the attendee to insert
    /// * `value` - The attendee to insert
    /// # Returns
    /// * `Result<Attendee, ApiError>` - The inserted attendee if successful, otherwise an error
    /// # Note
    /// Does check if a attendee with the same key already exists, if so returns an error
    fn insert_by_key(
        &mut self,
        key: Principal,
        value: Attendee,
    ) -> Result<(Principal, Attendee), ApiError> {
        self.store.with(|data| {
            if data.borrow().contains_key(&key) {
                return Err(ApiError::duplicate()
                    .add_method_name("insert_by_key")
                    .add_info(NAME)
                    .add_message("Key already exists"));
            }

            data.borrow_mut().insert(key, value.clone());
            Ok((key, value))
        })
    }

    /// Update a single attendee by key
    /// # Arguments
    /// * `key` - The user principal key of the attendee to update
    /// * `value` - The attendee to update
    /// # Returns
    /// * `Result<Attendee, ApiError>` - The updated attendee if successful, otherwise an error
    /// # Note
    /// Does check if a attendee with the same key already exists, if not returns an error
    fn update(
        &mut self,
        key: Principal,
        value: Attendee,
    ) -> Result<(Principal, Attendee), ApiError> {
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

    /// Remove a single attendee by key
    /// # Arguments
    /// * `key` - The user principal key of the attendee to remove
    /// # Returns
    /// * `bool` - True if the attendee was removed, otherwise false
    /// # Note
    /// TODO: Check if we want to do a soft delete
    fn remove(&mut self, key: Principal) -> bool {
        self.store
            .with(|data| data.borrow_mut().remove(&key).is_some())
    }
}
