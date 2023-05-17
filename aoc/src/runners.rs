use std::{
    collections::BTreeMap,
    fmt::Display,
    sync::{Mutex, Arc},
};

use lazy_static::lazy_static;

use crate::error::Error;

type Runner = dyn Fn() -> eyre::Result<String> + Send + Sync + 'static;

lazy_static! {
    pub(crate) static ref RUNNERS: Mutex<BTreeMap<(usize, usize, aoc_common::Part), Vec<(Option<String>, Arc<Runner>)>>> =
        Mutex::new(BTreeMap::new());
}

pub fn get_runner(year: usize, day: usize, part: aoc_common::Part, version: Option<String>) -> Result<Arc<Runner>, Error> {
    let map = RUNNERS.lock().unwrap();
    let keys = map.keys().cloned().collect::<Vec<_>>();

    for (y, d, p) in keys {
        if y == year && d == day && p == part {
            if let Some(alts) = map.get(&(year, day, part)) {
                let alts = alts.clone().into_iter().collect::<Vec<_>>();
                for (v, f) in alts {
                    if v == version {
                        return Ok(Arc::clone(&f));
                    }
                }
            }
        }
    }

    Err(Error::NotRegistered(year, day, part, version))
}

pub fn register_runner<F, T>(year: usize, day: usize, part: &str, version: Option<String>, func: F) -> eyre::Result<()>
where
    F: Fn() -> eyre::Result<T> + Send + Sync + 'static,
    T: Display,
{
    let mut map = RUNNERS.lock().unwrap();
    map.entry((year, day, part.parse()?))
        .or_insert_with(Vec::new)
        .push((version, Arc::new(move || func().map(|r| r.to_string()))));

    Ok(())
}
