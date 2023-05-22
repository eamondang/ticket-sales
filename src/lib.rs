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
  tickets_vip: UnorderedMap<u64, String>,
  tickets_premium: UnorderedMap<u64, String>,
  coupons: UnorderedMap<String, u32>,
  pub vip_price: Balance,
  pub premium_price: Balance,
  pub ticket_standard_saled: u64,
  pub ticket_vip_saled: u64,
  pub ticket_premium_saled: u64,
  pub buyers: LookupSet<AccountId>,
  pub buyer_ticket_links: UnorderedMap<AccountId, Vec<String>>,
}

#[near_bindgen]
impl Contract {
  #[init]
  pub fn new() -> Self {
    Self {
      owner_id: env::signer_account_id(),
      tickets_standard: UnorderedMap::new(b"tickets_free".to_vec()),
      tickets_vip: UnorderedMap::new(b"tickets_vip".to_vec()),
      tickets_premium: UnorderedMap::new(b"tickets_premium".to_vec()),
      coupons: UnorderedMap::new(b"coupons".to_vec()),
      vip_price: 0,
      premium_price: 0,
      ticket_standard_saled: 0,
      ticket_vip_saled: 0,
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
    assert_eq!(env::signer_account_id(), self.owner_id, "Only the owner get tickets.");
    let mut all_tickets = Vec::new();

    for key in 0..self.tickets_standard.len() {
      if let Some(link) = self.tickets_standard.get(&key) {
        all_tickets.push((key, link));
      }
    }

    all_tickets
  }

  pub fn add_tickets_vip(&mut self, ticket_links: Vec<String>) {
    assert_eq!(env::signer_account_id(), self.owner_id, "Only the owner can add tickets.");
    let mut key = self.tickets_vip.len();

    for link in ticket_links {
      self.tickets_vip.insert(&key, &link);
      key += 1;
    }
  }

  pub fn total_vip_tickets(&self) -> u64 {
    assert_eq!(env::signer_account_id(), self.owner_id, "Only the owner can add tickets.");
    self.tickets_vip.len()
  }

  pub fn get_all_tickets_vip(&self) -> Vec<(u64, String)> {
    assert_eq!(env::signer_account_id(), self.owner_id, "Only the owner get all tickets.");
    let mut all_tickets = Vec::new();

    for key in 0..self.tickets_vip.len() {
      if let Some(link) = self.tickets_vip.get(&key) {
        all_tickets.push((key, link));
      }
    }

    all_tickets
  }

  pub fn add_tickets_premium(&mut self, ticket_links: Vec<String>) {
    assert_eq!(env::signer_account_id(), self.owner_id, "Only the owner can add all tickets.");
    let mut key = self.tickets_premium.len();

    for link in ticket_links {
      self.tickets_premium.insert(&key, &link);
      key += 1;
    }
  }

  pub fn get_all_tickets_premium(&self) -> Vec<(u64, String)> {
    assert_eq!(env::signer_account_id(), self.owner_id, "Only the owner can get all tickets.");
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
    assert_eq!(env::signer_account_id(), self.owner_id, "Only the owner can get coupons.");
    self.coupons.get(&coupon_code)
  }

  // Get all coupons as a vector of tuples (coupon_code, discount)
  pub fn get_all_coupons(&self) -> Vec<(String, u32)> {
    assert_eq!(env::signer_account_id(), self.owner_id, "Only the owner can get all coupons.");
    self.coupons.iter().collect()
  }

  #[payable]
  pub fn purchase_vip_ticket(&mut self) -> Promise {
    let signer = env::signer_account_id();
    let key = self.ticket_vip_saled;
    let price = self.get_vip_price();

    assert!(env::attached_deposit() >= price, "Not enough deposit for the ticket.");
    assert!(!self.buyers.contains(&signer), "This wallet has already purchased a ticket.");
    assert!(self.ticket_vip_saled < 2000, "Ticket sale limit reached.");
    let ticket_link = self.tickets_vip.get(&key).expect("Ticket not available");

    self.tickets_vip.remove(&key);
    self.ticket_vip_saled += 1;
    self.buyers.insert(&signer);

    // Add the ticket link to the buyer_ticket_links map
    let mut buyer_links = self.buyer_ticket_links.get(&signer).unwrap_or_else(|| vec![]);
    buyer_links.push(ticket_link.clone());
    self.buyer_ticket_links.insert(&signer, &buyer_links);

    // Log the ticket link as an event
    let purchase_log: EventLog = EventLog {
      standard: "1.0.0".to_string(),
      event: EventLogVariant::Purchase(vec![PurchaseTicket {
        owner_id: signer.to_string(),
        price,
        ticket_link,
        memo: None,
      }]),
    };

    env::log_str(&purchase_log.to_string());
    Promise::new(self.owner_id.clone()).transfer(price)
  }

  pub fn ticket_premium_price(&mut self, price: Balance, near_price: f32) -> Balance {
    assert_eq!(env::signer_account_id(), self.owner_id, "Only the owner can Change Price.");
    let new_price = (price as f32 / near_price) as u128;
    self.premium_price = new_price * 10;
    new_price
  }

  pub fn get_premium_price(&self) -> Balance {
    self.premium_price
  }

  pub fn ticket_vip_price(&mut self, price: Balance, near_price: f32) {
    assert_eq!(env::signer_account_id(), self.owner_id, "Only the owner can Change Price.");
    let new_price = (price as f32 / near_price) as u128;
    self.vip_price = new_price;
  }

  pub fn get_vip_price(&self) -> Balance {
    self.vip_price
  }

  #[payable]
  pub fn purchase_premium_ticket(&mut self) -> Promise {
    let signer = env::signer_account_id();
    let key = self.ticket_premium_saled;
    let price = self.get_premium_price();

    assert!(env::attached_deposit() >= price, "Not enough deposit for the ticket.");
    assert!(self.tickets_premium.get(&key).is_some(), "Ticket not available.");
    assert!(self.ticket_premium_saled < 1000, "Ticket sale limit reached.");

    let ticket_link = self.tickets_premium.get(&key).expect("Ticket not available");
    self.tickets_premium.remove(&key);
    self.ticket_premium_saled += 1;

    let mut buyer_links = self.buyer_ticket_links.get(&signer).unwrap_or_else(|| vec![]);
    buyer_links.push(ticket_link.clone());
    self.buyer_ticket_links.insert(&signer, &buyer_links);

    // Log the ticket link as an event
    let purchase_log: EventLog = EventLog {
      standard: "1.0.0".to_string(),
      event: EventLogVariant::Purchase(vec![PurchaseTicket {
        owner_id: signer.to_string(),
        price,
        ticket_link,
        memo: None,
      }]),
    };

    env::log_str(&purchase_log.to_string());

    // Update this part to refund any extra deposit
    Promise::new(self.owner_id.clone()).transfer(price)
  }

  // Add this function to get a ticket link for a specific buyer
  pub fn get_ticket_links_by_buyer(&self, account_id: AccountId) -> Option<Vec<String>> {
    self.buyer_ticket_links.get(&account_id)
  }

  pub fn count_standard(&self) -> u64 {
    self.ticket_vip_saled
  }

  pub fn count_vipd(&self) -> u64 {
    self.ticket_vip_saled
  }

  pub fn count_premium(&self) -> u64 {
    self.ticket_premium_saled
  }
}
