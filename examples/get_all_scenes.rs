extern crate philipshue;
use std::env;
use philipshue::bridge::Bridge;
use philipshue::hue::AppData;

mod discover;
use discover::discover;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage : {:?} <username>", args[0]);
        return;
    }
    let bridge = Bridge::new(discover().pop().unwrap(), args.remove(1));

    match bridge.get_all_scenes() {
        Ok(scenes) => {
            let name_len = scenes.values().map(|s| s.name.len()).chain(Some(4)).max().unwrap();
            let id_len = scenes.keys().map(|id| id.len()).chain(Some(2)).max().unwrap();
            println!("{0:2$} {1:3$} recycle locked appdata_and_version lights",
                     "id",
                     "name",
                     id_len,
                     name_len,
            );
            for (id, scene) in scenes.into_iter() {
                println!("{:id_len$} {:name_len$} {:7} {:6} {:20?} {:?}",
                         id,
                         scene.name,
                         scene.recycle,
                         scene.locked,
                         Show(scene.appdata.map(|AppData{data, version}| (data, version))),
                         scene.lights,
                         id_len = id_len,
                         name_len = name_len);
            }
        }
        Err(err) => println!("Error: {}", err),
    }
}

use std::fmt::{self, Debug, Display};

struct Show<T>(Option<T>);

impl<T: Debug> Debug for Show<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Some(ref x) => x.fmt(f),
            _ => Display::fmt("N/A", f),
        }
    }
}
