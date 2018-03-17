/*
* @Author: BlahGeek
* @Date:   2017-04-18
* @Last Modified by:   BlahGeek
* @Last Modified time: 2018-03-17
*/

mod utils;

mod linux_desktop_entry;
mod search_engine;
mod file_browser;
mod custom_script;
mod youdao;
mod clipboard;
mod reload;

use std::fmt;
use std::error::Error;

use mcore::config::Config;

#[derive(Clone)]
#[derive(Debug)]
struct ActionError {
    reason: String,
}

impl ActionError {
    fn new(reason: &str) -> ActionError {
        ActionError { reason: reason.into() }
    }
}

impl Error for ActionError {
    fn description(&self) -> &str {
        "ActionError"
    }
}
impl fmt::Display for ActionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}

use mcore::item::Item;

pub fn get_action_items(config: &Config) -> Vec<Item> {
    let mut ret : Vec<Item> = vec![];

    ret.append(&mut search_engine::get(config));
    ret.append(&mut file_browser::get(config));
    ret.append(&mut linux_desktop_entry::get(config));
    ret.append(&mut custom_script::get(config));

    ret.push(clipboard::get(config));
    ret.push(youdao::get(config));
    ret.push(reload::get(config));

    ret
}
