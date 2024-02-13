use std::thread::LocalKey;

use candid::Principal;
use ic_cdk::caller;

use super::storage_api::{IdentifierRefMethods, PrincipalIdentifier, StorageMethods, StorageRef};
use canister_types::models::{
    api_error::ApiError,
    identifier::{Identifier, IdentifierKind},
    profile::Profile,
};

pub struct ProfileStore<'a> {
    store: &'a LocalKey<StorageRef<Principal, Profile>>,
    identifier_ref: &'a LocalKey<StorageRef<PrincipalIdentifier, Principal>>,
}

impl<'a> ProfileStore<'a> {
    pub fn new(
        store: &'a LocalKey<StorageRef<Principal, Profile>>,
        identifier_ref: &'a LocalKey<StorageRef<PrincipalIdentifier, Principal>>,
    ) -> Self {
        Self {
            store,
            identifier_ref,
        }
    }
}

pub const NAME: &str = "profiles";

impl IdentifierRefMethods<PrincipalIdentifier> for ProfileStore<'static> {
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

impl StorageMethods<Principal, Profile> for ProfileStore<'static> {
    /// Get a single user profile by key
    /// # Arguments
    /// * `key` - The key of the profile to get
    /// # Returns
    /// * `Result<Profile, ApiError>` - The profile if found, otherwise an error
    fn get(&self, key: Principal) -> Result<(Principal, Profile), ApiError> {
        self.store.with(|data| {
            data.borrow()
                .get(&key)
                .ok_or(ApiError::not_found().add_method_name("get").add_info(NAME))
                .map(|value| (key, value))
        })
    }

    /// Get multiple profiles by key
    /// # Arguments
    /// * `ids` - The keys of the profiles to get
    /// # Returns
    /// * `Vec<Profile>` - The reports if found, otherwise an empty vector
    fn get_many(&self, keys: Vec<Principal>) -> Vec<(Principal, Profile)> {
        self.store.with(|data| {
            let mut profiles = Vec::new();
            for key in keys {
                if let Some(profile) = data.borrow().get(&key) {
                    profiles.push((key, profile));
                }
            }
            profiles
        })
    }

    /// Find a single user profile by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Option<(Principal, Profile)>` - The profile if found, otherwise None
    fn find<F>(&self, filter: F) -> Option<(Principal, Profile)>
    where
        F: Fn(&Principal, &Profile) -> bool,
    {
        self.store.with(|data| {
            data.borrow()
                .iter()
                .find(|(id, profile)| filter(id, profile))
                .map(|(key, profile)| (key, profile))
        })
    }

    /// Find all user profiles by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Vec<(Principal, Profile)>` - The profiles if found, otherwise an empty vector
    fn filter<F>(&self, filter: F) -> Vec<(Principal, Profile)>
    where
        F: Fn(&Principal, &Profile) -> bool,
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
    fn insert(&mut self, _value: Profile) -> Result<(Principal, Profile), ApiError> {
        Err(ApiError::unsupported()
            .add_method_name("insert") // value should be `insert` as a string value
            .add_info(NAME)
            .add_message("This value requires a key to be inserted, use `insert_by_key` instead"))
    }

    /// Insert a single user profile by key
    /// # Arguments
    /// * `key` - The user principal as key of the profile to insert
    /// * `value` - The profile to insert
    /// # Returns
    /// * `Result<Profile, ApiError>` - The inserted profile if successful, otherwise an error
    /// # Note
    /// Does check if a profile with the same key already exists, if so returns an error
    fn insert_by_key(
        &mut self,
        key: Principal,
        value: Profile,
    ) -> Result<(Principal, Profile), ApiError> {
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

    /// Update a single user profile by key
    /// # Arguments
    /// * `key` - The user principal key of the profile to update
    /// * `value` - The profile to update
    /// # Returns
    /// * `Result<Profile, ApiError>` - The updated profile if successful, otherwise an error
    /// # Note
    /// Does check if a profile with the same key already exists, if not returns an error
    fn update(&mut self, key: Principal, value: Profile) -> Result<(Principal, Profile), ApiError> {
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

    /// Remove a single user profile by key
    /// # Arguments
    /// * `key` - The user principal key of the profile to remove
    /// # Returns
    /// * `bool` - True if the profile was removed, otherwise false
    /// # Note
    /// TODO: Check if we want to do a soft delete
    fn remove(&mut self, key: Principal) -> bool {
        self.store
            .with(|data| data.borrow_mut().remove(&key).is_some())
    }
}
