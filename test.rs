
//! Rust kernel module test
use core::{stringify, fmt};

use kernel::prelude::pr_info;
use kernel::prelude::CStr;
use kernel::prelude::{ThisModule, String, Result, Box};
use kernel::prelude::module;
use kernel::linked_list::{List, Links, GetLinks};

module! {
    type: RustOutOfTree,
    name: b"test",
    author: b"test",
    description: b"Rust kernel nodule test",
    license: b"GPL v2",
}

struct RustOutOfTree {
    message: String,
}
#[allow(dead_code)]
struct Data{
    name: u32,
    link: Links<Data>
}

impl Data{
    pub(crate) fn new(name: u32)->Box<Data>{
        Box::try_new(Data{name, link: Links::<Data>::new()}).unwrap()
    }
}
impl fmt::Display for Data{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl GetLinks for Data{
    type EntryType = Data;
    fn get_links(s: &<Self as GetLinks>::EntryType) -> &kernel::linked_list::Links<<Self as GetLinks>::EntryType> { 
        &s.link
    }
}
impl kernel::Module for RustOutOfTree {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust module (init)\n");
        pr_info!("{}\n", stringify!(RustOutOfTree));
        let mut x = List::<Box<Data>>::new();
        for i in 0..100{
            x.push_back(Data::new(i as u32));
        }
        for _ in 0..100
        {
            pr_info!("{}", x.pop_front().unwrap());
        }
        pr_info!("{}", x.is_empty());
        Ok(RustOutOfTree {
            message: "on the heap!".try_to_owned()?,
        })
    }
}

impl Drop for RustOutOfTree {
    fn drop(&mut self) {
        pr_info!("My message is {}\n", self.message);
        pr_info!("Rust module (exit)\n");
    }
}
