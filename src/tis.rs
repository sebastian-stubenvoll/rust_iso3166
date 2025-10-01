use phf::phf_map;
use phf::Map;

use super::*;

#[derive(Debug, Ord, PartialOrd, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TerritoryType {
    Geographical,
    Economical,
    Political,
}

#[derive(Debug, Ord, PartialOrd, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TisCode {
    Territory(&'static TerritoryCode),
    Country(&'static CountryCode),
}

#[derive(Debug, Ord, PartialOrd, Copy, Clone, PartialEq, Eq, Hash)]
pub struct TerritoryCode {
    pub name: &'static str,
    pub tis_a: &'static str,
    pub territory_type: TerritoryType,
    pub numeric: i32,
    pub parents: Vec<TisCode>,
    pub children: Vec<TisCode>,
}

impl TerritoryCode {
    pub fn resolve_countries(&self) -> Vec<CountryCode> {
        match tis_code {
            TisCode::Country(c) => vec![c],
            TisCode::Territory(t) => t
                .children
                .iter()
                .flat_map(|child| self.resolve_recursive(child))
                .collect(),
        }
    }
}

pub fn from_numeric_str(numeric: &str) -> Option<TerritoryCode> {
    todo!();
}
