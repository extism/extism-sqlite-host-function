use extism_pdk::{*, config::get_memory, bindings::extism_load};
use json::error;
use serde::{Serialize, Deserialize};

extern "C" {
    fn execute_sql(ptr: i64) -> i64;
}

#[derive(Deserialize)]
struct Event {
    pub account_id: String,
}

#[derive(Deserialize, Clone)]
struct Account {
    account_id: String,
    username: String,
    email: String,
}

type AccountList = Vec<Account>;

fn fetch_account(account_id: String) -> Option<Account> {
    let sql = format!("SELECT * FROM accounts WHERE account_id = '{}' LIMIT 1", account_id);
    let memory = Memory::from_bytes(&sql);
    let offset = unsafe { execute_sql(memory.offset as i64) } as u64;
    let length = unsafe { extism_pdk::bindings::extism_length(offset) };
    let memory = Memory { offset, length, free: false };
    let data = memory.to_vec(); 
    let json_string = String::from_utf8(data).expect("Not valid UTF-8");
    let accounts: AccountList = serde_json::from_str(&json_string).unwrap();
    if let Some(a) = accounts.first() {
        Some(a.clone())
    } else {
        None
    }
}

#[plugin_fn]
pub fn on_event(Json(event): Json<Event>) -> FnResult<String> {
    // we could do some logic here for this event, maybe send an email?
    // just returning the data for now
    if let Some(a) = fetch_account(event.account_id) {
        Ok(format!("Got event for user {} {}", a.username, a.email))
    } else {
        Ok("Could not find account".into())
    }
}