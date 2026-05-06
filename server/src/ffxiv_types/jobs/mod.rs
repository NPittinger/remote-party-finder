//! Job types

use serde::Serialize;
use serde::Deserialize;

pub mod classification;
pub mod combat;
pub mod non_combat;

pub use crate::ffxiv_types::jobs::classification::Classification;
pub use crate::ffxiv_types::jobs::combat::{Class, Job};
pub use crate::ffxiv_types::jobs::non_combat::NonCombatJob;
use crate::ffxiv_types::roles::Role;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClassJob {
    Class(Class),
    Job(Job),
    NonCombat(NonCombatJob),
}

impl ClassJob {
    pub fn as_job(&self) -> Option<Job> {
        match self {
            Self::Job(j) => Some(*j),
            _ => None,
        }
    }

    pub fn as_class(&self) -> Option<Class> {
        match self {
            Self::Class(c) => Some(*c),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Job(j) => j.as_str(),
            Self::Class(c) => c.as_str(),
            Self::NonCombat(nc) => nc.as_str(),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Job(j) => j.name(),
            Self::Class(c) => c.name(),
            Self::NonCombat(nc) => nc.name(),
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Self::Job(j) => j.code(),
            Self::Class(c) => c.code(),
            Self::NonCombat(nc) => nc.code(),
        }
    }

    pub fn role(&self) -> Option<Role> {
        match self {
            Self::Job(j) => Some(j.role()),
            Self::Class(c) => Some(c.role()),
            _ => None,
        }
    }

    pub fn classification(&self) -> Classification {
        match self {
            Self::Job(j) => j.classification(),
            Self::Class(c) => c.classification(),
            Self::NonCombat(nc) => nc.classification(),
        }
    }
}
