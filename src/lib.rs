use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen, PanicOnDefault};
near_sdk::setup_alloc!();

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Writing {
  pub text: String,
  pub sender: String,
  pub receiver: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
  pub writing_list: Vector<Writing>, //This line gives err
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
  WritingList,
}

#[near_bindgen]
impl Contract {
  pub fn add(&mut self) -> String {
    let w = Writing {
      text: "val".to_string(),
      sender: "val".to_string(),
      receiver: "val".to_string(),
    };

    let _ = &self.writing_list.push(&w);

    return "Added".to_string();
  }
  pub fn get_writings(&self) -> u64 {
    return self.writing_list.len();
  }

  #[init]
  pub fn new_contract() -> Self {
    let this = Self {
      writing_list: Vector::new(StorageKey::WritingList.try_to_vec().unwrap())
    };

    this
  }
}