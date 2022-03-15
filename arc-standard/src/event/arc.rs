use crate::*;

use std::fmt;

pub const EVENT_ARC_METADATA_SPEC: &str = "1.0.0";

pub const EVENT_ARC_STANDARD_ACTOR: &str = "actor";
pub const EVENT_ARC_STANDARD_GUILD: &str = "guild";

/// Enum that represents the data type for ArcEventLog.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[serde(crate = "near_sdk::serde")]
#[non_exhaustive]
pub enum ArcEventVariant {
    ArcRegister(Vec<ArcRegisterLog>),
}

/// Interface to capture data about an event
///
/// Arguments:
/// * `module`: name e.g. actor, guild, ...
/// * `version`: e.g. 1.0.0
/// * `event`: associate event data
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ArcEventLog {
    pub module: String,
    pub version: String,

    #[serde(flatten)]
    pub event: ArcEventVariant,
}

impl fmt::Display for ArcEventLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "EVENT_ARC:{}",
            &serde_json::to_string(self).map_err(|_| fmt::Error)?
        ))
    }
}

/// An event log to capture registration
///
/// Arguments
/// * `user_id`: "account.near"
/// * `keys_list`: ["1", "abc"]
/// * `memo`: optional message
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ArcRegisterLog {
    pub user_id: String,
    pub keys_list: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arc_format_register_no_memo() {
        let expected = r#"EVENT_ARC:{"module":"actor","version":"1.0.0","event":"arc_register","data":[{"user_id":"vecno.near","keys_list":["story-arc",".io"]}]}"#;
        let log = ArcEventLog {
            module: EVENT_ARC_STANDARD_ACTOR.to_string(),
            version: EVENT_ARC_METADATA_SPEC.to_string(),
            event: ArcEventVariant::ArcRegister(vec![ArcRegisterLog {
                user_id: "vecno.near".to_owned(),
                keys_list: vec!["story-arc".to_string(), ".io".to_string()],
                memo: None,
            }]),
        };
        assert_eq!(expected, log.to_string());
    }
    #[test]
    fn arc_format_register_some_memo() {
        let expected = r#"EVENT_ARC:{"module":"name","version":"1.0.1","event":"arc_register","data":[{"user_id":"story-arc.near","keys_list":["vecno",".io"],"memo":"Go Team! Go!"}]}"#;
        let log = ArcEventLog {
            module: "name".to_string(),
            version: "1.0.1".to_string(),
            event: ArcEventVariant::ArcRegister(vec![ArcRegisterLog {
                user_id: "story-arc.near".to_owned(),
                keys_list: vec!["vecno".to_string(), ".io".to_string()],
                memo: Some("Go Team! Go!".to_owned()),
            }]),
        };
        assert_eq!(expected, log.to_string());
    }
}
