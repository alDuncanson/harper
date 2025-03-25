use serde::{Deserialize, Serialize};

use super::Error;
use super::affix_replacement::{AffixReplacement, HumanReadableAffixReplacement};
use crate::WordMetadata;

#[derive(Debug, Clone)]
pub struct Expansion {
    /// If `!true`, this is a prefix
    /// But if `true` it may be a prefix but may be a property only
    pub suffix_or_property: bool,
    pub cross_product: bool,
    pub replacements: Vec<AffixReplacement>,
    /// When the expansion is applied, the resulting word will have this
    /// metadata appended to it.
    pub target_metadata: WordMetadata,
    /// When the expansion is applied, the __parent__ word will have this
    /// metadata appended to it.
    pub base_metadata: WordMetadata,
}

impl Expansion {
    pub fn into_human_readable(self) -> HumanReadableExpansion {
        HumanReadableExpansion {
            suffix_or_property: self.suffix_or_property,
            cross_product: self.cross_product,
            replacements: self
                .replacements
                .iter()
                .map(AffixReplacement::to_human_readable)
                .collect(),
            target_metadata: self.target_metadata,
            base_metadata: self.base_metadata,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanReadableExpansion {
    pub suffix_or_property: bool,
    pub cross_product: bool,
    pub replacements: Vec<HumanReadableAffixReplacement>,
    pub target_metadata: WordMetadata,
    pub base_metadata: WordMetadata,
}

impl HumanReadableExpansion {
    pub fn into_normal(self) -> Result<Expansion, Error> {
        let mut replacements = Vec::with_capacity(self.replacements.len());

        for replacement in &self.replacements {
            replacements.push(replacement.to_normal()?);
        }

        Ok(Expansion {
            suffix_or_property: self.suffix_or_property,
            cross_product: self.cross_product,
            replacements,
            target_metadata: self.target_metadata,
            base_metadata: self.base_metadata,
        })
    }
}
