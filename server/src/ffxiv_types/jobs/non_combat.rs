//! Non-combat job types

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

use serde::{Serialize, Deserialize};

use crate::ffxiv_types::errors::UnknownVariant;
use crate::ffxiv_types::jobs::classification::Classification;

/// The Disciple of the Land and Disciple of the Hand jobs available in the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NonCombatJob {
    // Gatherers
    Botanist,
    Fisher,
    Miner,

    // Crafters
    Alchemist,
    Armorer,
    Blacksmith,
    Carpenter,
    Culinarian,
    Goldsmith,
    Leatherworker,
    Weaver,
}

impl NonCombatJob {
    pub const ALL: [NonCombatJob; 11] = [
        NonCombatJob::Botanist,
        NonCombatJob::Fisher,
        NonCombatJob::Miner,
        NonCombatJob::Alchemist,
        NonCombatJob::Armorer,
        NonCombatJob::Blacksmith,
        NonCombatJob::Carpenter,
        NonCombatJob::Culinarian,
        NonCombatJob::Goldsmith,
        NonCombatJob::Leatherworker,
        NonCombatJob::Weaver,
    ];

    pub fn as_str(&self) -> &'static str {
        match *self {
            NonCombatJob::Botanist => "Botanist",
            NonCombatJob::Fisher => "Fisher",
            NonCombatJob::Miner => "Miner",

            NonCombatJob::Alchemist => "Alchemist",
            NonCombatJob::Armorer => "Armorer",
            NonCombatJob::Blacksmith => "Blacksmith",
            NonCombatJob::Carpenter => "Carpenter",
            NonCombatJob::Culinarian => "Culinarian",
            NonCombatJob::Goldsmith => "Goldsmith",
            NonCombatJob::Leatherworker => "Leatherworker",
            NonCombatJob::Weaver => "Weaver",
        }
    }

    pub fn name(&self) -> &'static str {
        // if any variants with spaces are added, this must be changed
        self.as_str()
    }

    pub fn code(&self) -> &'static str {
        match *self {
            NonCombatJob::Botanist => "BTN",
            NonCombatJob::Fisher => "FSH",
            NonCombatJob::Miner => "MIN",

            NonCombatJob::Alchemist => "ALC",
            NonCombatJob::Armorer => "ARM",
            NonCombatJob::Blacksmith => "BSM",
            NonCombatJob::Carpenter => "CRP",
            NonCombatJob::Culinarian => "CUL",
            NonCombatJob::Goldsmith => "GSM",
            NonCombatJob::Leatherworker => "LTW",
            NonCombatJob::Weaver => "WVR",
        }
    }

    pub fn classification(&self) -> Classification {
        match *self {
            NonCombatJob::Botanist
            | NonCombatJob::Fisher
            | NonCombatJob::Miner => Classification::Land,

            NonCombatJob::Alchemist
            | NonCombatJob::Armorer
            | NonCombatJob::Blacksmith
            | NonCombatJob::Carpenter
            | NonCombatJob::Culinarian
            | NonCombatJob::Goldsmith
            | NonCombatJob::Leatherworker
            | NonCombatJob::Weaver => Classification::Hand,
        }
    }
}

impl FromStr for NonCombatJob {
    type Err = UnknownVariant;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let job = match s.to_lowercase().as_str() {
            "botanist" | "btn" => NonCombatJob::Botanist,
            "fisher" | "fsh" => NonCombatJob::Fisher,
            "miner" | "min" => NonCombatJob::Miner,

            "alchemist" | "alc" => NonCombatJob::Alchemist,
            "armorer" | "arm" => NonCombatJob::Armorer,
            "blacksmith" | "bsm" => NonCombatJob::Blacksmith,
            "carpenter" | "crp" => NonCombatJob::Carpenter,
            "culinarian" | "cul" => NonCombatJob::Culinarian,
            "goldsmith" | "gsm" => NonCombatJob::Goldsmith,
            "leatherworker" | "ltw" => NonCombatJob::Leatherworker,
            "weaver" | "wvr" => NonCombatJob::Weaver,
            _ => return Err(UnknownVariant("NonCombatJob", s.into()))
        };

        Ok(job)
    }
}

impl Display for NonCombatJob {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.name())
    }
}
