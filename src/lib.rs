use near_sdk::{near_bindgen, borsh};

#[near_bindgen]
#[derive(Default, borsh::BorshDeserialize, borsh::BorshSerialize)]
#[borsh(crate = "near_sdk::borsh")]
pub struct MyApp;

#[near_bindgen]
impl MyApp {
    pub fn set_account_avatar(&mut self, avatar_url: String) {
        let account_id = near_sdk::env::predecessor_account_id();
        near_sdk::env::storage_write(account_id.as_bytes(), avatar_url.as_bytes());
    }

    pub fn get_account_avatar(&self, account_id: String) -> Option<String> {
        near_sdk::env::storage_read(account_id.as_bytes()).map(|bytes| String::from_utf8(bytes).unwrap())
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
