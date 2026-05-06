//! Types for use in FFXIV-related projects.

pub mod clans;
pub mod data_centers;
pub mod errors;
pub mod guardians;
pub mod jobs;
pub mod races;
pub mod roles;
pub mod worlds;

pub use crate::ffxiv_types::clans::Clan;
pub use crate::ffxiv_types::data_centers::DataCenter;
pub use crate::ffxiv_types::guardians::Guardian;
pub use crate::ffxiv_types::jobs::Job;
pub use crate::ffxiv_types::jobs::NonCombatJob;
pub use crate::ffxiv_types::jobs::Classification;
pub use crate::ffxiv_types::races::Race;
pub use crate::ffxiv_types::roles::Role;
pub use crate::ffxiv_types::worlds::World;
