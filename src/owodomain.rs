use crate::error::*;
use derive_more::Display;
use reqwest::Client;
use std::{collections::HashSet, str::Lines};

#[derive(Debug, PartialEq, Eq, Hash, Display, Clone)]
pub enum OwoDomain {
    #[display(fmt = "{}", _0)]
    Standard(String),

    #[display(fmt = "*.{}", _0)]
    Wildcard(String),
}

#[derive(Debug)]
pub struct OwoDomainChecker {
    reqwest_client: Client,
}

impl OwoDomainChecker {
    pub fn new(reqwest_client: impl Into<Option<Client>>) -> Self {
        OwoDomainChecker {
            reqwest_client: reqwest_client.into().unwrap_or_else(|| Client::new()),
        }
    }

    pub fn check_domain(&self, domain: &str) -> Result<()> {
        // Ok(()) is only used to indicate no errors happened
        // An Err would contain any error, showing both failure and the error
        use std::io::Read as _;

        let response: reqwest::Response = self
            .reqwest_client
            .get(&format!(
                "https://{}/{}",
                domain,
                crate::consts::SUCCESS_FILE
            ))
            .send()?;

        if !response.status().is_success() {
            return Err(CheckError::UnsuccessfulCheck(response.status().as_u16()).into());
        }

        for (i, b) in response.bytes().enumerate() {
            if b.ok() != crate::consts::SUCCESS_TEXT.get(i).cloned() {
                return Err(CheckError::UnequalResponse.into());
            }
        }

        Ok(())
    }
}

pub fn parse_domain_list(lines: Lines<'_>) -> HashSet<OwoDomain> {
    let mut v = HashSet::with_capacity({
        let hint = lines.size_hint();
        hint.1.unwrap_or_else(move || hint.0)
    });
    for line in lines {
        if let Some(s) = parse_domain(line) {
            v.insert(s);
        }
    }
    v.shrink_to_fit();
    v
}

fn parse_domain(domain: &str) -> Option<OwoDomain> {
    if domain.is_empty() {
        return None;
    }

    if domain.starts_with('#') {
        return None;
    }

    let domain = domain
        .trim_start_matches("default-files:")
        .trim_start_matches("default-links:");

    if domain.starts_with("*.") {
        Some(OwoDomain::Wildcard(
            domain.trim_start_matches("*.").to_owned(),
        ))
    } else {
        Some(OwoDomain::Standard(domain.to_owned()))
    }
}

impl serde::Serialize for OwoDomain {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}
