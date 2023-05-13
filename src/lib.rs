use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  collections::{LookupSet, UnorderedMap},
  env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise,
};

mod event;
pub use crate::event::*;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
  pub owner_id: AccountId,
  tickets_standard: UnorderedMap<u64, String>,
  tickets_elite: UnorderedMap<u64, String>,
  tickets_premium: UnorderedMap<u64, String>,
  coupons: UnorderedMap<String, u32>,
  info: UnorderedMap<String, String>,
  pub premium_price: Balance,
  pub ticket_standard_saled: u64,
  pub ticket_elite_saled: u64,
  pub ticket_premium_saled: u64,
  pub buyers: LookupSet<AccountId>,
  pub buyer_ticket_links: UnorderedMap<AccountId, String>,
}

#[near_bindgen]
impl Contract {
  #[init]
  pub fn new() -> Self {
    Self {
      owner_id: env::signer_account_id(),
      tickets_standard: UnorderedMap::new(b"tickets_free".to_vec()),
      tickets_elite: UnorderedMap::new(b"tickets_elite".to_vec()),
      tickets_premium: UnorderedMap::new(b"tickets_premium".to_vec()),
      coupons: UnorderedMap::new(b"coupons".to_vec()),
      info: UnorderedMap::new(b"info".to_vec()),
      premium_price: 0,
      ticket_standard_saled: 0,
      ticket_elite_saled: 0,
      ticket_premium_saled: 0,
      buyers: LookupSet::new(b"buyers".to_vec()),
      buyer_ticket_links: UnorderedMap::new(b"buyer_ticket".to_vec()),
    }
  }

  pub fn add_tickets_standard(&mut self, ticket_links: Vec<String>) {
    assert_eq!(env::signer_account_id(), self.owner_id, "Only the owner can add tickets.");
    let mut key = self.tickets_standard.len();

    for link in ticket_links {
      self.tickets_standard.insert(&key, &link);
      key += 1;
    }
  }

  pub fn get_all_tickets_standard(&self) -> Vec<(u64, String)> {
    assert_eq!(env::signer_account_id(), self.owner_id, "Only the owner can add coupons.");
    let mut all_tickets = Vec::new();

    for key in 0..self.tickets_standard.len() {
      if let Some(link) = self.tickets_standard.get(&key) {
        all_tickets.push((key, link));
      }
    }

    all_tickets
  }

  pub fn add_tickets_elite(&mut self, ticket_links: Vec<String>) {
    assert_eq!(env::signer_account_id(), self.owner_id, "Only the owner can add tickets.");
    let mut key = self.tickets_elite.len();

    for link in ticket_links {
      self.tickets_elite.insert(&key, &link);
      key += 1;
    }
  }

  pub fn get_all_tickets_elite(&self) -> Vec<(u64, String)> {
    assert_eq!(env::signer_account_id(), self.owner_id, "Only the owner can add coupons.");
    let mut all_tickets = Vec::new();

    for key in 0..self.tickets_elite.len() {
      if let Some(link) = self.tickets_elite.get(&key) {
        all_tickets.push((key, link));
      }
    }

    all_tickets
  }

  pub fn add_tickets_premium(&mut self, ticket_links: Vec<String>) {
    assert_eq!(env::signer_account_id(), self.owner_id, "Only the owner can add tickets.");
    let mut key = self.tickets_premium.len();

    for link in ticket_links {
      self.tickets_premium.insert(&key, &link);
      key += 1;
    }
  }

  pub fn get_all_tickets_premium(&self) -> Vec<(u64, String)> {
    assert_eq!(env::signer_account_id(), self.owner_id, "Only the owner can add coupons.");
    let mut all_tickets = Vec::new();

    for key in 0..self.tickets_premium.len() {
      if let Some(link) = self.tickets_premium.get(&key) {
        all_tickets.push((key, link));
      }
    }

    all_tickets
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

  pub fn get_all_info(&self) -> Vec<(String, String)> {
    assert_eq!(env::signer_account_id(), self.owner_id, "Only the owner can add coupons.");
    self.info.iter().collect()
  }

  pub fn purchase_elite_ticket(&mut self, email: Option<String>, telephone: Option<String>) {
    let signer = env::signer_account_id();
    let key = self.ticket_elite_saled;

    if email.is_some() & telephone.is_none() {
      self.info.insert(&email.unwrap(), &"".to_string());
    } else if email.is_none() & telephone.is_some() {
      self.info.insert(&"".to_string(), &telephone.unwrap());
    } else {
      self.info.insert(&email.unwrap(), &telephone.unwrap());
    }

    assert!(!self.buyers.contains(&signer), "This wallet has already purchased a ticket.");
    assert!(self.ticket_elite_saled < 2000, "Ticket sale limit reached.");

    let ticket_link = self.tickets_elite.get(&key).expect("Ticket not available");
    self.tickets_elite.remove(&key);
    self.ticket_elite_saled += 1;
    self.buyers.insert(&signer);

    // Add the ticket link to the buyer_ticket_links map
    self.buyer_ticket_links.insert(&signer, &ticket_link);

    // Log the ticket link as an event
    let purchase_log: EventLog = EventLog {
      standard: "1.0.0".to_string(),
      event: EventLogVariant::Purchase(vec![PurchaseTicket { owner_id: signer.to_string(), ticket_link, memo: None }]),
    };

    env::log_str(&purchase_log.to_string());
  }

  pub fn ticket_premium_price(&mut self, price: Balance, near_price: f32) {
    assert_eq!(env::signer_account_id(), self.owner_id, "Only the owner can add coupons.");
    let new_price = (price as f32 / near_price) as u128;
    self.premium_price = new_price;
  }

  pub fn get_ticket_price(&self) -> Balance {
    self.premium_price
  }

  #[payable]
  pub fn purchase_premium_ticket(&mut self, email: Option<String>, telephone: Option<String>) -> Promise {
    let signer = env::signer_account_id();
    let key = self.ticket_premium_saled;
    let price = self.get_ticket_price();

    if email.is_some() & telephone.is_none() {
      self.info.insert(&email.unwrap(), &"".to_string());
    } else if email.is_none() & telephone.is_some() {
      self.info.insert(&"".to_string(), &telephone.unwrap());
    } else {
      self.info.insert(&email.unwrap(), &telephone.unwrap());
    }

    assert!(env::attached_deposit() >= price, "Not enough deposit for the ticket.");
    assert!(self.tickets_premium.get(&key).is_some(), "Ticket not available.");
    assert!(self.ticket_premium_saled < 1000, "Ticket sale limit reached.");

    let ticket_link = self.tickets_premium.get(&key).expect("Ticket not available");
    self.tickets_premium.remove(&key);
    self.ticket_premium_saled += 1;

    // Add the ticket link to the buyer_ticket_links map
    self.buyer_ticket_links.insert(&signer, &ticket_link);

    // Log the ticket link as an event
    let purchase_log: EventLog = EventLog {
      standard: "1.0.0".to_string(),
      event: EventLogVariant::Purchase(vec![PurchaseTicket { owner_id: signer.to_string(), ticket_link, memo: None }]),
    };

    env::log_str(&purchase_log.to_string());

    Promise::new(self.owner_id.clone()).transfer(price)
  }

  // Add this function to get a ticket link for a specific buyer
  pub fn get_ticket_link_by_buyer(&self, account_id: AccountId) -> Option<String> {
    self.buyer_ticket_links.get(&account_id)
  }

  pub fn count_standard(&self) -> u64 {
    self.ticket_elite_saled
  }

  pub fn count_elited(&self) -> u64 {
    self.ticket_elite_saled
  }

  pub fn count_premium(&self) -> u64 {
    self.ticket_premium_saled
  }
}
