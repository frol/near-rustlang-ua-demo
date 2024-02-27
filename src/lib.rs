use near_sdk::{near_bindgen, borsh, AccountId};

#[near_bindgen]
#[derive(Default, borsh::BorshDeserialize, borsh::BorshSerialize)]
#[borsh(crate = "near_sdk::borsh")]
pub struct MyApp {
    avatar_urls: std::collections::HashMap<AccountId, String>,
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
}

// #[cfg(test)]
// mod tests {
//     fn test_set_account_avatar() {
//         let mut contract = super::MyApp;
//         contract.set_account_avatar("alice".to_string(), "https://example.com/avatar.png".to_string());
//         assert_eq!(contract.get_account_avatar("alice".to_string()), Some("https://example.com/avatar.png".to_string()));
//     }
// }
