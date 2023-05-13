use std::fmt;

use near_sdk::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[serde(crate = "near_sdk::serde")]
#[non_exhaustive]
pub enum EventLogVariant {
  Purchase(Vec<PurchaseTicket>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct PurchaseTicket {
  pub owner_id: String,
  pub ticket_link: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub memo: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct EventLog {
  pub standard: String,

  // `flatten` to not have "event": {<EventLogVariant>} in the JSON, just have the contents of {<EventLogVariant>}.
  #[serde(flatten)]
  pub event: EventLogVariant,
}

impl fmt::Display for EventLog {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_fmt(format_args!("EVENT_JSON:{}", &serde_json::to_string(self).map_err(|_| fmt::Error)?))
  }
}
