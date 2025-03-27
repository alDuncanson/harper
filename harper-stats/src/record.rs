use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

use harper_core::linting::{LintGroupConfig, LintKind};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
pub struct Record {
    pub kind: RecordKind,
    /// Recorded as seconds from the Unix Epoch
    pub when: u64,
    pub uuid: Uuid,
}

impl Record {
    /// Record a new instance at the current system time.
    pub fn now(kind: RecordKind) -> Result<Self, SystemTimeError> {
        Ok(Self {
            kind,
            when: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
            uuid: Uuid::new_v4(),
        })
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
pub enum RecordKind {
    Lint(LintKind),
    LintConfigUpdate(LintGroupConfig),
}

#[cfg(test)]
mod tests {
    use harper_core::linting::{LintGroupConfig, LintKind};
    use quickcheck::Arbitrary;

    use super::{Record, RecordKind};

    fn arbitrary_lintconfig(g: &mut quickcheck::Gen) -> LintGroupConfig {
        let mut config = LintGroupConfig::default();

        for _ in 0..g.size() {
            config.set_rule_enabled(String::arbitrary(g), bool::arbitrary(g));
        }

        config
    }

    impl Arbitrary for RecordKind {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let lcu = Self::LintConfigUpdate(arbitrary_lintconfig(g));

            g.choose(&[
                Self::Lint(LintKind::Spelling),
                Self::Lint(LintKind::Capitalization),
                Self::Lint(LintKind::Style),
                Self::Lint(LintKind::Formatting),
                Self::Lint(LintKind::Repetition),
                Self::Lint(LintKind::Enhancement),
                Self::Lint(LintKind::Readability),
                Self::Lint(LintKind::WordChoice),
                Self::Lint(LintKind::Miscellaneous),
                lcu,
            ])
            .unwrap()
            .clone()
        }
    }

    impl Arbitrary for Record {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            Record {
                kind: RecordKind::arbitrary(g),
                when: u64::arbitrary(g),
                uuid: uuid::Builder::from_u128(u128::arbitrary(g)).into_uuid(),
            }
        }
    }
}
