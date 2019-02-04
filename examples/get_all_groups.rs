extern crate philipshue;
use std::env;
use philipshue::bridge::Bridge;

mod discover;
use discover::discover;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage : {:?} <username>", args[0]);
        return;
    }
    let bridge = Bridge::new(discover().pop().unwrap(), args.remove(1));

    match bridge.get_all_groups() {
        Ok(groups) => {
            let name_len = groups.values().map(|l| l.name.len()).chain(Some(4)).max().unwrap();
            let type_len = groups.values()
                .map(|l| l.group_type.to_string().len())
                .chain(Some(4))
                .max()
                .unwrap();
            println!("id {0:1$} {2:3$} class        any_on all_on bri lights",
                     "name",
                     name_len,
                     "type",
                     type_len);
            for (id, group) in groups.iter() {
                println!("{:2} {:name_len$} {:type_len$} {:12} {:6} {:6} {:3} {:?}",
                         id,
                         group.name,
                         group.group_type,
                         Show(&group.class),
                         Show(&group.state.as_ref().map(|s| s.any_on)),
                         Show(&group.state.as_ref().map(|s| s.all_on)),
                         Show(&group.state.as_ref().and_then(|s| s.bri)),
                         group.lights,
                         name_len = name_len,
                         type_len = type_len);
            }
        }
        Err(err) => println!("Error: {}", err),
    }
}

use std::fmt::{self, Display, Debug};

struct Show<'a, T: 'a>(&'a Option<T>);

impl<'a, T: 'a + Display> Display for Show<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            Some(ref x) => x.fmt(f),
            _ => Display::fmt("N/A", f),
        }
    }
}

impl<'a, T: 'a + Debug> Debug for Show<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            Some(ref x) => x.fmt(f),
            _ => Display::fmt("N/A", f),
        }
    }
}
