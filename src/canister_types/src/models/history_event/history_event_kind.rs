use std::str::FromStr;

use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum HistoryEventKind {
    GroupRoleChanged,
}

impl std::fmt::Display for HistoryEventKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HistoryEventKind::GroupRoleChanged => write!(f, "group_role_changed"),
        }
    }
}

impl From<HistoryEventKind> for String {
    fn from(kind: HistoryEventKind) -> Self {
        kind.to_string()
    }
}

impl FromStr for HistoryEventKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "group_role_changed" => Ok(HistoryEventKind::GroupRoleChanged),
            _ => Err(format!("Unknown history event kind: {}", s)),
        }
    }
}
