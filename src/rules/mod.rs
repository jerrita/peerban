use serde_json::Value;

use crate::peer::Peer;

pub mod preload;

#[derive(Debug, Clone)]
pub enum RuleType {
    IDPrefixMatch,
    IDContains,
    NamePrefixMatch,
    NameContains,
    IPBlockMatch,
    ProgressProbe,
    ExcessiveProbe,
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub class: RuleType,
    pub value: Value,
}

impl From<&str> for RuleType {
    fn from(s: &str) -> Self {
        match s {
            "idStartsWith" => RuleType::IDPrefixMatch,
            "idContains" => RuleType::IDContains,
            "nameStartsWith" => RuleType::NamePrefixMatch,
            "nameContains" => RuleType::NameContains,
            "ipBlockMatch" => RuleType::IPBlockMatch,
            "progressProbe" => RuleType::ProgressProbe,
            "excessiveProbe" => RuleType::ExcessiveProbe,
            _ => panic!("Invalid rule type."),
        }
    }
}

impl Rule {
    pub fn match_peer(&self, peer: &Peer, torrent_size: u64) -> bool {
        match self.class {
            RuleType::IDPrefixMatch => peer.id.starts_with(&self.value.as_str().unwrap()),
            RuleType::IDContains => peer.id.contains(&self.value.as_str().unwrap()),
            RuleType::NamePrefixMatch => peer.name.starts_with(&self.value.as_str().unwrap()),
            RuleType::NameContains => peer.name.contains(&self.value.as_str().unwrap()),
            RuleType::ProgressProbe => peer.uploaded as f64 / torrent_size as f64 - peer.progress > self.value.as_f64().unwrap(),
            RuleType::ExcessiveProbe => peer.downloaded as f64 / torrent_size as f64 > self.value.as_f64().unwrap(),
            RuleType::IPBlockMatch => unimplemented!("IPBlockMatch is not implemented yet.")
        }
    }
}


impl From<&str> for Rule {
    fn from(s: &str) -> Self {
        let mut split = s.split('@');
        if split.clone().count() != 2 {
            panic!("Invalid rule string, use class@value format.");
        }
        Rule {
            class: split.next().unwrap().into(),
            value: split.next().unwrap().into(),
        }
    }
}