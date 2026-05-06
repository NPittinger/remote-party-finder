//! Data center types

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

use serde::{Serialize, Deserialize};

use crate::ffxiv_types::errors::UnknownVariant;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataCenter {
    Aether,
    Chaos,
    Crystal,
    Dynamis,
    Elemental,
    Gaia,
    Light,
    Mana,
    Materia,
    Meteor,
    Primal,
    Shadow,
}

impl DataCenter {
    pub const ALL: [DataCenter; 12] = [
        DataCenter::Aether,
        DataCenter::Chaos,
        DataCenter::Crystal,
        DataCenter::Dynamis,
        DataCenter::Elemental,
        DataCenter::Gaia,
        DataCenter::Light,
        DataCenter::Mana,
        DataCenter::Materia,
        DataCenter::Meteor,
        DataCenter::Primal,
        DataCenter::Shadow,
    ];

    pub fn as_str(&self) -> &'static str {
        match *self {
            DataCenter::Aether => "Aether",
            DataCenter::Chaos => "Chaos",
            DataCenter::Crystal => "Crystal",
            DataCenter::Dynamis => "Dynamis",
            DataCenter::Elemental => "Elemental",
            DataCenter::Gaia => "Gaia",
            DataCenter::Light => "Light",
            DataCenter::Mana => "Mana",
            DataCenter::Materia => "Materia",
            DataCenter::Meteor => "Meteor",
            DataCenter::Primal => "Primal",
            DataCenter::Shadow => "Shadow"
        }
    }

    pub fn name(&self) -> &'static str {
        // if any variants with spaces are added, this must be changed
        self.as_str()
    }
}

impl FromStr for DataCenter {
    type Err = UnknownVariant;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data_center = match s.to_lowercase().as_str() {
            "aether" => DataCenter::Aether,
            "chaos" => DataCenter::Chaos,
            "crystal" => DataCenter::Crystal,
            "dynamis" => DataCenter::Dynamis,
            "elemental" => DataCenter::Elemental,
            "gaia" => DataCenter::Gaia,
            "light" => DataCenter::Light,
            "mana" => DataCenter::Mana,
            "materia" => DataCenter::Materia,
            "meteor" => DataCenter::Meteor,
            "primal" => DataCenter::Primal,
            "shadow" => DataCenter::Shadow,
            _ => return Err(UnknownVariant("DataCenter", s.into()))
        };

        Ok(data_center)
    }
}

impl Display for DataCenter {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.name())
    }
}
