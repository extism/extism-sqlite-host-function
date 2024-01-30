use extism_pdk::{bindings::extism_load, config::get_memory, *};
use json::error;
use serde::{Deserialize, Serialize};

#[host_fn]
extern "ExtismHost" {
    fn execute_sql(s: &str) -> Json<AccountList>;
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

fn fetch_account(account_id: String) -> Result<Option<Account>, Error> {
    let sql = format!(
        "SELECT * FROM accounts WHERE account_id = '{}' LIMIT 1",
        account_id
    );
    let Json(accounts) = unsafe { execute_sql(&sql)? };
    if let Some(a) = accounts.first() {
        Ok(Some(a.clone()))
    } else {
        Ok(None)
    }
}

#[plugin_fn]
pub fn on_event(Json(event): Json<Event>) -> FnResult<String> {
    // we could do some logic here for this event, maybe send an email?
    // just returning the data for now
    if let Some(a) = fetch_account(event.account_id)? {
        Ok(format!("Got event for user {} {}", a.username, a.email))
    } else {
        Ok("Could not find account".into())
    }
}
