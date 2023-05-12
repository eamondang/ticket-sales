use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  collections::{LookupSet, UnorderedMap},
  env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise,
};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
  pub owner_id: AccountId,
  pub tickets: UnorderedMap<u64, String>,
  pub coupons: UnorderedMap<String, u32>,
  pub ticket_saled: u64,
  pub price: Balance,
  pub buyers: LookupSet<AccountId>,
}

#[near_bindgen]
impl Contract {
  #[init]
  pub fn new(price: Balance) -> Self {
    Self {
      owner_id: env::signer_account_id(),
      tickets: UnorderedMap::new(b"tickets".to_vec()),
      coupons: UnorderedMap::new(b"coupons".to_vec()),
      ticket_saled: 0,
      price,
      buyers: LookupSet::new(b"buyers".to_vec()),
    }
  }

  pub fn add_tickets(&mut self, ticket_links: Vec<String>) {
    assert_eq!(env::signer_account_id(), self.owner_id, "Only the owner can add tickets.");
    let mut key = self.tickets.len();

    for link in ticket_links {
      self.tickets.insert(&key, &link);
      key += 1;
    }
  }
  pub fn get_ticket(&self, key: u64) -> Option<String> {
    self.tickets.get(&key)
  }

  pub fn get_all_tickets(&self) -> Vec<(u64, String)> {
    let mut all_tickets = Vec::new();

    for key in 0..self.tickets.len() {
      if let Some(link) = self.tickets.get(&key) {
        all_tickets.push((key, link));
      }
    }

    all_tickets
  }

  pub fn set_price(&mut self, new_price: Balance) {
    assert_eq!(env::signer_account_id(), self.owner_id, "Only the owner can set the price.");
    self.price = new_price;
  }

  pub fn get_price(&self) -> Balance {
    self.price
  }

  pub fn add_coupon(&mut self, code: String, discount: u32) {
    assert_eq!(env::signer_account_id(), self.owner_id, "Only the owner can add coupons.");
    self.coupons.insert(&code, &discount);
  }

  // Get a single coupon by its code
  pub fn get_coupon(&self, coupon_code: String) -> Option<u32> {
    assert_eq!(env::signer_account_id(), self.owner_id, "Only the owner can add coupons.");
    self.coupons.get(&coupon_code)
  }

  // Get all coupons as a vector of tuples (coupon_code, discount)
  pub fn get_all_coupons(&self) -> Vec<(String, u32)> {
    assert_eq!(env::signer_account_id(), self.owner_id, "Only the owner can add coupons.");
    self.coupons.iter().collect()
  }

  #[payable]
  pub fn purchase_ticket(&mut self, key: u64, coupon_code: Option<String>) -> Promise {
    let signer = env::signer_account_id();
    let mut price = self.price;

    if let Some(code) = coupon_code {
      if let Some(discount) = self.coupons.get(&code) {
        price = price * (100 - discount as u128) / 100;
      }
    }

    assert!(!self.buyers.contains(&signer), "This wallet has already purchased a ticket.");
    assert!(env::attached_deposit() >= price, "Not enough deposit for the ticket.");
    assert!(self.tickets.get(&key).is_some(), "Ticket not available.");
    assert!(self.ticket_saled < 2000, "Ticket sale limit reached.");

    self.tickets.remove(&key);
    self.ticket_saled += 1;
    self.buyers.insert(&signer);

    if price > 0 {
      let refund_amount = env::attached_deposit() - price;

      if refund_amount > 0 {
        Promise::new(signer).transfer(refund_amount)
      } else {
        Promise::new(self.owner_id.clone()).transfer(price)
      }
    } else {
      Promise::new(signer).transfer(0) // No need to transfer any funds if the price is 0
    }
  }

  pub fn count(&self) -> u64 {
    self.ticket_saled
  }
}
