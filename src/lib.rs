use near_sdk::{near_bindgen, borsh, AccountId};

#[near_bindgen]
#[derive(borsh::BorshDeserialize, borsh::BorshSerialize)]
#[borsh(crate = "near_sdk::borsh")]
pub struct MyApp {
    #[allow(deprecated)]
    avatar_urls: near_sdk::store::UnorderedMap<AccountId, String>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            avatar_urls: near_sdk::store::UnorderedMap::new(b"avatars:".to_vec()),
        }
    }
}

#[near_bindgen]
impl MyApp {
    pub fn set_account_avatar(&mut self, avatar_url: String) {
        let account_id = near_sdk::env::predecessor_account_id();
        self.avatar_urls.insert(account_id, avatar_url);
    }

    pub fn get_account_avatar(&self, account_id: AccountId) -> Option<String> {
        self.avatar_urls.get(&account_id).cloned()
    }

    pub fn migrate() {
        near_sdk::assert_self();
        near_sdk::env::state_write(&Self {
            avatar_urls: near_sdk::store::UnorderedMap::new(b"avatars:".to_vec()),
        });
    }
}
