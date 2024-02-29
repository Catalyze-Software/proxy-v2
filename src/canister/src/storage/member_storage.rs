use super::storage_api::{
    IdentifierRefMethods, PrincipalIdentifier, StorageMethods, MEMBERS, MEMBERS_IDENTIFIER_REF,
};
use candid::Principal;
use canister_types::models::{
    api_error::ApiError,
    identifier::{Identifier, IdentifierKind},
    member::Member,
};
use ic_cdk::caller;

pub struct MemberStore;

pub const NAME: &str = "members";

impl IdentifierRefMethods<PrincipalIdentifier> for MemberStore {
    /// get a new identifier
    /// # Returns
    /// * `PrincipalIdentifier` - The new identifier
    fn new_identifier() -> PrincipalIdentifier {
        let id = MEMBERS_IDENTIFIER_REF.with(|data| {
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
    fn get_id_by_identifier(key: &PrincipalIdentifier) -> Option<Principal> {
        MEMBERS_IDENTIFIER_REF.with(|data| data.borrow().get(key))
    }

    /// Get the identifier by key
    /// # Arguments
    /// * `value` - The value to get the identifier for
    /// # Returns
    /// * `Option<PrincipalIdentifier>` - The identifier if found, otherwise None
    fn get_identifier_by_id(value: &Principal) -> Option<PrincipalIdentifier> {
        MEMBERS_IDENTIFIER_REF.with(|data| {
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
    fn insert_identifier_ref(key: PrincipalIdentifier) -> Result<Principal, ApiError> {
        MEMBERS_IDENTIFIER_REF.with(|data| {
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
    fn remove_identifier_ref(key: &PrincipalIdentifier) -> bool {
        MEMBERS_IDENTIFIER_REF.with(|data| data.borrow_mut().remove(key).is_some())
    }
}

impl StorageMethods<Principal, Member> for MemberStore {
    /// Get a single member by key
    /// # Arguments
    /// * `key` - The key of the member to get
    /// # Returns
    /// * `Result<Member, ApiError>` - The member if found, otherwise an error
    fn get(key: Principal) -> Result<(Principal, Member), ApiError> {
        MEMBERS.with(|data| {
            data.borrow()
                .get(&key)
                .ok_or(ApiError::not_found().add_method_name("get").add_info(NAME))
                .map(|value| (key, value))
        })
    }

    /// Get multiple members by key
    /// # Arguments
    /// * `ids` - The keys of the members to get
    /// # Returns
    /// * `Vec<Group>` - The groups if found, otherwise an empty vector
    fn get_many(keys: Vec<Principal>) -> Vec<(Principal, Member)> {
        MEMBERS.with(|data| {
            let mut members = Vec::new();
            for key in keys {
                if let Some(member) = data.borrow().get(&key) {
                    members.push((key, member));
                }
            }
            members
        })
    }

    /// Find a single member by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Option<(Principal, Member)>` - The member if found, otherwise None
    fn find<F>(filter: F) -> Option<(Principal, Member)>
    where
        F: Fn(&Principal, &Member) -> bool,
    {
        MEMBERS.with(|data| {
            data.borrow()
                .iter()
                .find(|(id, value)| filter(id, value))
                .map(|(key, value)| (key, value))
        })
    }

    /// Find all members by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Vec<(Principal, Member)>` - The members if found, otherwise an empty vector
    fn filter<F>(filter: F) -> Vec<(Principal, Member)>
    where
        F: Fn(&Principal, &Member) -> bool,
    {
        MEMBERS.with(|data| {
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
    fn insert(_value: Member) -> Result<(Principal, Member), ApiError> {
        Err(ApiError::unsupported()
            .add_method_name("insert") // value should be `insert` as a string value
            .add_info(NAME)
            .add_message("This value requires a key to be inserted, use `insert_by_key` instead"))
    }

    /// Insert a single member by key
    /// # Arguments
    /// * `key` - The user principal as key of the member to insert
    /// * `value` - The member to insert
    /// # Returns
    /// * `Result<Member, ApiError>` - The inserted member if successful, otherwise an error
    /// # Note
    /// Does check if a member with the same key already exists, if so returns an error
    fn insert_by_key(key: Principal, value: Member) -> Result<(Principal, Member), ApiError> {
        MEMBERS.with(|data| {
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

    /// Update a single member by key
    /// # Arguments
    /// * `key` - The user principal key of the member to update
    /// * `value` - The member to update
    /// # Returns
    /// * `Result<Member, ApiError>` - The updated member if successful, otherwise an error
    /// # Note
    /// Does check if a member with the same key already exists, if not returns an error
    fn update(key: Principal, value: Member) -> Result<(Principal, Member), ApiError> {
        MEMBERS.with(|data| {
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

    /// Remove a single member by key
    /// # Arguments
    /// * `key` - The user principal key of the member to remove
    /// # Returns
    /// * `bool` - True if the member was removed, otherwise false
    /// # Note
    fn remove(key: Principal) -> bool {
        MEMBERS.with(|data| data.borrow_mut().remove(&key).is_some())
    }
}
