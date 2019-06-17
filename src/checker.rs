use crate::owodomain::OwoDomain;
use parking_lot::RwLock;
use std::{collections::HashMap, sync::Arc};

pub fn checker(
    rwlock: Arc<RwLock<HashMap<OwoDomain, bool>>>,
    checker: crate::owodomain::OwoDomainChecker,
) {
    loop {
        let mut clone = {
            let lock = rwlock.read();
            lock.clone()
        };
        for (key, val) in clone.iter_mut() {
            let key = match key {
                OwoDomain::Standard(s) => s.to_owned(),
                OwoDomain::Wildcard(s) => format!("owochecker.{}", s),
            };
            match checker.check_domain(&key) {
                Ok(()) => *val = true,
                Err(_) => *val = false,
            }
        }
        {
            *rwlock.write() = clone;
        }

        std::thread::sleep(std::time::Duration::from_secs(60 * 30));
    }
}
