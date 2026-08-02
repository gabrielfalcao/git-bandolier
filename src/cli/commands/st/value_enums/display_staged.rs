use std::fmt::Display;

use clap::builder::PossibleValue;
use clap::ValueEnum;
use heck::{
    ToKebabCase, ToLowerCamelCase, ToPascalCase, ToSnakeCase, ToTrainCase,
};
use serde::{Deserialize, Serialize};

#[derive(
    Clone,
    Debug,
    Copy,
    Default,
    PartialOrd,
    Ord,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum DisplayStaged
{
    /// do not display staged files at all
    #[default]
    None,

    // #[clap(help = "display all staged files")]
    /// only display staged files whose status match the status-related flags of the `st` command
    Matching,

    // #[clap(help = "display all staged files")]
    /// display all staged files
    All,

    // // #[clap(help = "only display staged files that were added")]
    // /// only display staged files that were added
    // OnlyAdded,

    // // #[clap(help = "only display staged files that were modified")]
    // /// only display staged files that were modified
    // OnlyModified,

    // // #[clap(help = "only display staged files that were deleted")]
    // /// only display staged files that were deleted
    // OnlyDeleted,
}
impl DisplayStaged
{
    pub fn variant_name_snake(&self) -> &'static str
    {
        match self
        {
            DisplayStaged::None => "none",
            DisplayStaged::All => "all",
            DisplayStaged::OnlyAdded => "only_added",
            DisplayStaged::OnlyModified => "only_modified",
            DisplayStaged::OnlyDeleted => "only_deleted",
        }
    }

    pub fn variants<'a>() -> &'a [DisplayStaged]
    {
        &[
            DisplayStaged::None,
            DisplayStaged::All,
            DisplayStaged::OnlyAdded,
            DisplayStaged::OnlyModified,
            DisplayStaged::OnlyDeleted,
        ]
    }

    pub fn variant_name_kebab(&self) -> String
    {
        self.variant_name_snake().to_kebab_case()
    }

    pub fn variant_name_pascal(&self) -> String
    {
        self.variant_name_snake().to_pascal_case()
    }

    pub fn variant_name_train(&self) -> String
    {
        self.variant_name_snake().to_train_case()
    }

    fn to_possible_strings(&self) -> [String; 4]
    {
        [
            self.variant_name_snake().to_string(),
            self.variant_name_kebab(),
            self.variant_name_pascal(),
            self.variant_name_train(),
        ]
    }
}

impl ValueEnum for DisplayStaged
{
    fn value_variants<'a>() -> &'a [DisplayStaged]
    {
        DisplayStaged::variants()
    }

    fn to_possible_value(&self) -> Option<PossibleValue>
    {
        Some(
            PossibleValue::new(self.to_string())
                .alias(self.variant_name_kebab())
                .alias(self.variant_name_pascal())
                .alias(self.variant_name_train()),
        )
    }

    fn from_str(
        val: &str,
        ignore_case: bool,
    ) -> std::result::Result<DisplayStaged, String>
    {
        let val = if ignore_case
        {
            val.to_lowercase()
        }
        else
        {
            val.to_string()
        };
        let val = val.trim();
        for (variant, possible_strings) in DisplayStaged::variants()
            .iter()
            .map(|variant| (variant, variant.to_possible_strings()))
        {
            for pos in possible_strings
            {
                if pos == val
                {
                    return Ok(*variant);
                }
            }
        }
        return Err(val.to_string());
    }
}
