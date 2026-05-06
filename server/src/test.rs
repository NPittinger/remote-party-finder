use crate::listing::{
    ConditionFlags, DutyCategory, DutyFinderSettingsFlags, DutyType, JobFlags, LootRuleFlags,
    ObjectiveFlags, PartyFinderListing, PartyFinderSlot, SearchAreaFlags,
};
use sestring::SeString;

const LISTING: &str = r###"
{
  "id": 36254,
  "content_id_lower": 1271300023,
  "name": "VGVzdCBOYW1l",
  "description": "VGhpcyBpcyBteSB0ZXN0IGRlc2NyaXB0aW9uLg==",
  "created_world": 37,
  "home_world": 57,
  "current_world": 37,
  "category": 32,
  "duty": 1026,
  "duty_type": 2,
  "beginners_welcome": false,
  "seconds_remaining": 3549,
  "min_item_level": 999,
  "num_parties": 1,
  "slots_available": 8,
  "last_server_restart": 1777351794,
  "objective": 8,
  "conditions": 2,
  "duty_finder_settings": 0,
  "loot_rules": 2,
  "search_area": 33,
  "slots": [
    {
      "accepting": 131072
    },
    {
      "accepting": 16
    },
    {
      "accepting": 1280
    },
    {
      "accepting": 69206016
    },
    {
      "accepting": 4202496
    },
    {
      "accepting": 1351092736
    },
    {
      "accepting": 135270400
    },
    {
      "accepting": 4294967294
    }
  ],
  "jobs_present": [
    28,
    4,
    0,
    0,
    0,
    0,
    0,
    0
  ]
}"###;

lazy_static::lazy_static! {
    static ref EXPECTED: PartyFinderListing = PartyFinderListing {
        id: 36254,
        content_id_lower: 1271300023,
        name: SeString::parse(b"Test Name").unwrap(),
        description: SeString::parse(b"This is my test description.").unwrap(),
        created_world: 37,
        home_world: 57,
        current_world: 37,
        category: DutyCategory::Raid,
        duty: 1026,
        duty_type: DutyType::Normal,
        beginners_welcome: false,
        seconds_remaining: 3549,
        min_item_level: 999,
        num_parties: 1,
        slots_available: 8,
        last_server_restart: 1777351794,
        objective: ObjectiveFlags::LOOT,
        conditions: ConditionFlags::DUTY_COMPLETE,
        duty_finder_settings: DutyFinderSettingsFlags::NONE,
        loot_rules: LootRuleFlags::LOOTMASTER,
        search_area: SearchAreaFlags::DATA_CENTRE | SearchAreaFlags::ONE_PLAYER_PER_JOB,
        slots: vec![
            PartyFinderSlot {
                accepting: JobFlags::SCHOLAR
            },
            PartyFinderSlot {
                accepting: JobFlags::LANCER
            },
            PartyFinderSlot {
                accepting: JobFlags::PALADIN | JobFlags::WARRIOR
            },
            PartyFinderSlot {
                accepting: JobFlags::DARK_KNIGHT | JobFlags::GUNBREAKER
            },
            PartyFinderSlot {
                accepting: JobFlags::WHITE_MAGE | JobFlags::ASTROLOGIAN
            },
            PartyFinderSlot {
                accepting: JobFlags::MONK | JobFlags::DRAGOON | JobFlags::NINJA | JobFlags::SAMURAI | JobFlags::REAPER | JobFlags::VIPER
            },
            PartyFinderSlot {
                accepting: JobFlags::BARD | JobFlags::MACHINIST | JobFlags::DANCER
            },
            PartyFinderSlot {
                accepting: JobFlags::GLADIATOR | JobFlags::PUGILIST | JobFlags::MARAUDER | JobFlags::LANCER | JobFlags::ARCHER | JobFlags::CONJURER | JobFlags::THAUMATURGE | JobFlags::PALADIN | JobFlags::MONK | JobFlags::WARRIOR | JobFlags::DRAGOON | JobFlags::BARD | JobFlags::WHITE_MAGE | JobFlags::BLACK_MAGE | JobFlags::ARCANIST | JobFlags::SUMMONER | JobFlags::SCHOLAR | JobFlags::ROGUE | JobFlags::NINJA | JobFlags::MACHINIST | JobFlags::DARK_KNIGHT | JobFlags::ASTROLOGIAN | JobFlags::SAMURAI | JobFlags::RED_MAGE | JobFlags::BLUE_MAGE | JobFlags::GUNBREAKER | JobFlags::DANCER | JobFlags::REAPER | JobFlags::SAGE | JobFlags::VIPER | JobFlags::PICTOMANCER // TODO: add Beastmaster when added to Dalamud's JobFlags
            },
        ],
        jobs_present: vec![28, 4, 0, 0, 0, 0, 0, 0],
    };
}

#[test]
fn deserialise_listing() {
    let listing: PartyFinderListing = serde_json::from_str(LISTING).unwrap();
    assert_eq!(listing, *EXPECTED,)
}

#[test]
fn serialise_listing() {
    assert_eq!(
        serde_json::to_string_pretty(&*EXPECTED).unwrap(),
        LISTING.trim(),
    );
}
