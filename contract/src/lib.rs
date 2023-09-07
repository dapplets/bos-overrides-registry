use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;
use near_sdk::{env, near_bindgen};

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Mutation {
    description: String,
    overrides: Vec<Override>,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Override {
    from_src: String,
    to_src: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct MutationRegistry {
    mutations: UnorderedMap<AccountId, UnorderedMap<String, Mutation>>,
}

impl Default for MutationRegistry {
    fn default() -> Self {
        Self {
            mutations: UnorderedMap::new(b"mutations".to_vec()),
        }
    }
}

#[near_bindgen]
impl MutationRegistry {
    pub fn create_mutation(
        &mut self,
        id: String,
        description: String,
        overrides: Vec<Override>,
    ) -> bool {
        let author_id = env::signer_account_id();
        let new_mutation = Mutation {
            description,
            overrides,
        };

        let mut author_mutations = self.mutations.get(&author_id).unwrap_or_else(|| {
            UnorderedMap::new(format!("mutations-{}", author_id).as_bytes().to_vec())
        });

        author_mutations.insert(&id, &new_mutation);
        self.mutations.insert(&author_id, &author_mutations);

        true
    }

    pub fn update_mutation(
        &mut self,
        id: String,
        new_description: Option<String>,
        new_overrides: Option<Vec<Override>>,
    ) {
        let author_id = env::signer_account_id();
        if let Some(mut author_mutations) = self.mutations.get(&author_id) {
            if let Some(mut mutation) = author_mutations.get(&id) {
                if let Some(description) = new_description {
                    mutation.description = description;
                }

                if let Some(overrides) = new_overrides {
                    mutation.overrides = overrides;
                }

                author_mutations.insert(&id, &mutation);
            }
        }
    }

    pub fn get_mutation(&self, author_id: AccountId, id: String) -> Option<Mutation> {
        let author_mutations = self.mutations.get(&author_id)?;
        author_mutations.get(&id)
    }

    pub fn get_all_mutations(&self) -> Vec<(AccountId, String, Mutation)> {
        let mut all_mutations = Vec::new();

        for author_id in self.mutations.keys() {
            if let Some(author_mutations) = self.mutations.get(&author_id) {
                for id in author_mutations.keys() {
                    if let Some(mutation) = author_mutations.get(&id) {
                        all_mutations.push((author_id.clone(), id, mutation));
                    }
                }
            }
        }

        all_mutations
    }

    pub fn get_mutations_by_author(&self, author: AccountId) -> Vec<(String, Mutation)> {
        let mut mutations_vec = Vec::new();

        if let Some(author_mutations) = self.mutations.get(&author) {
            for id in author_mutations.keys() {
                if let Some(mutation) = author_mutations.get(&id) {
                    mutations_vec.push((id, mutation));
                }
            }
        }

        mutations_vec
    }
}
