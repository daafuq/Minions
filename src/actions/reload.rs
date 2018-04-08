extern crate nix;

use std::sync::Arc;

use mcore::action::{Action, ActionResult};
use mcore::item::{Item, Icon};
use mcore::config::Config;
use mcore::errors::*;


struct ReloadAction {}

impl Action for ReloadAction {
    fn runnable_bare(&self) -> bool { true }
    fn run_bare(&self) -> ActionResult {
        nix::sys::signal::kill(0, nix::sys::signal::SIGHUP)
            .map_err(|e| Error::with_chain(e, "Failed to send SIGHUP to myself"))?;
        Ok(Vec::new())
    }
}

pub fn get(_: &Config) -> Item {
    let mut item = Item::new("Reload All Actions");
    item.subtitle = Some("Equivalent to `kill -HUP 0`".into());
    item.badge = Some("Minions".into());
    item.priority = 100;
    item.icon = Some(Icon::FontAwesome("cog".into()));
    item.action = Some(Arc::new(ReloadAction{}));
    item
}
