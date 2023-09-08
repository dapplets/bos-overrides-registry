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
        author_id: AccountId,
        mutation_id: String,
        description: String,
        overrides: Vec<Override>,
    ) -> bool {
        if env::predecessor_account_id() != author_id {
            env::panic_str("Mutations: permission denied");
        }

        let new_mutation = Mutation {
            description,
            overrides,
        };

        let mut author_mutations = self.mutations.get(&author_id).unwrap_or_else(|| {
            UnorderedMap::new(format!("mutations-{}", author_id).as_bytes().to_vec())
        });

        author_mutations.insert(&mutation_id, &new_mutation);
        self.mutations.insert(&author_id, &author_mutations);

        true
    }

    pub fn update_mutation(
        &mut self,
        author_id: AccountId,
        mutation_id: String,
        description: Option<String>,
        overrides: Option<Vec<Override>>,
    ) {
        if env::predecessor_account_id() != author_id {
            env::panic_str("Mutations: permission denied");
        }

        if let Some(mut author_mutations) = self.mutations.get(&author_id) {
            if let Some(mut mutation) = author_mutations.get(&mutation_id) {
                if let Some(description) = description {
                    mutation.description = description;
                }

                if let Some(overrides) = overrides {
                    mutation.overrides = overrides;
                }

                author_mutations.insert(&mutation_id, &mutation);
            }
        }
    }

    pub fn copy_overrides(
        &mut self,
        source_author_id: AccountId,
        source_mutation_id: String,
        target_author_id: AccountId,
        target_mutation_id: String
    ) {
        if env::predecessor_account_id() != target_author_id {
            env::panic_str("Mutations: permission denied");
        }
        
        // Get source mutations
        let source_mutations = self.mutations.get(&source_author_id);
        let source_mutation = match source_mutations {
            Some(mutations) => mutations.get(&source_mutation_id),
            None => None
        };
    
        let source_overrides = match source_mutation {
            Some(mutation) => mutation.overrides,
            None => {
                env::panic_str("Source mutation not found");
            }
        };
    
        // Get target mutation
        let mut target_mutations = self.mutations.get(&target_author_id).unwrap_or_else(|| {
            UnorderedMap::new(format!("mutations-{}", target_author_id).as_bytes().to_vec())
        });
    
        if let Some(mut target_mutation) = target_mutations.get(&target_mutation_id) {
            target_mutation.overrides = source_overrides;
            target_mutations.insert(&target_mutation_id, &target_mutation);
            self.mutations.insert(&target_author_id, &target_mutations);
        } else {
            env::panic_str("Target mutation not found");
        }
    }

    pub fn get_mutation(&self, author_id: AccountId, mutation_id: String) -> Option<Mutation> {
        let author_mutations = self.mutations.get(&author_id)?;
        author_mutations.get(&mutation_id)
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

    pub fn get_mutations_by_author(&self, author_id: AccountId) -> Vec<(String, Mutation)> {
        let mut mutations_vec = Vec::new();

        if let Some(author_mutations) = self.mutations.get(&author_id) {
            for id in author_mutations.keys() {
                if let Some(mutation) = author_mutations.get(&id) {
                    mutations_vec.push((id, mutation));
                }
            }
        }

        mutations_vec
    }
}
