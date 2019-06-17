#![feature(proc_macro_hygiene, decl_macro)]

mod checker;
pub mod consts;
pub mod error;
pub mod owodomain;
mod routes;

use self::error::*;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

fn main() -> Result<(), exitfailure::ExitFailure> {
    let reqwest_client = reqwest::Client::builder()
        .use_rustls_tls()
        .default_headers({
            use reqwest::header::*;
            let mut map = HeaderMap::with_capacity(1);
            map.insert(
                "User-Agent",
                concat!(
                    "OwoChecker (https://github.com/Proximyst/owochecker, ",
                    env!("CARGO_PKG_VERSION"),
                    ")"
                )
                .parse()?,
            );
            map
        })
        .timeout(std::time::Duration::from_secs(3))
        .build()?;
    let domain_list = {
        let mut response = reqwest_client.get(self::consts::DOMAIN_LIST_URL).send()?;
        if !response.status().is_success() {
            return Err(StartError::DomainListError(response.status().as_u16()).into());
        }

        let set = self::owodomain::parse_domain_list(response.text()?.lines());
        let mut map = HashMap::with_capacity(set.len());
        for i in set {
            map.insert(i, false);
        }
        Arc::new(RwLock::new(map))
    };

    {
        let checker = Arc::clone(&domain_list);
        std::thread::spawn(move || {
            self::checker::checker(
                checker,
                self::owodomain::OwoDomainChecker::new(reqwest_client),
            )
        });
    }

    use rocket::routes;

    Err(StartError::LaunchError(
        rocket::ignite()
            .manage(domain_list)
            .mount("/", routes![routes::check_domains])
            .launch(),
    )
    .into())
}
