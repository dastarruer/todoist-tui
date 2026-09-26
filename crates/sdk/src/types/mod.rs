pub mod project;
pub mod task;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(pub String);

impl From<Uuid> for Id {
    fn from(value: Uuid) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for Id {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for Id {
    fn from(value: &str) -> Self {
        Self::from(String::from(value))
    }
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Uid(pub String);

/// Represents a Todoist color option.
///
/// See the [official Todoist API docs](https://developer.todoist.com/api/v1/#tag/Colors)
/// for more info.
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum Color {
    BerryRed,
    Red,
    Orange,
    Yellow,
    OliveGreen,
    LimeGreen,
    Green,
    MintGreen,
    Teal,
    SkyBlue,
    LightBlue,
    Blue,
    Grape,
    Violet,
    Lavender,
    Magenta,
    Salmon,
    #[default]
    Charcoal,
    Grey,
    Taupe,
}

impl Color {
    /// Returns the ID of the color as per
    /// [Todoist's API docs](https://developer.todoist.com/api/v1/#tag/Colors).
    #[must_use]
    pub const fn id(&self) -> u8 {
        match self {
            Self::BerryRed => 30,
            Self::Red => 31,
            Self::Orange => 32,
            Self::Yellow => 33,
            Self::OliveGreen => 34,
            Self::LimeGreen => 35,
            Self::Green => 36,
            Self::MintGreen => 37,
            Self::Teal => 38,
            Self::SkyBlue => 39,
            Self::LightBlue => 40,
            Self::Blue => 41,
            Self::Grape => 42,
            Self::Violet => 43,
            Self::Lavender => 44,
            Self::Magenta => 45,
            Self::Salmon => 46,
            Self::Charcoal => 47,
            Self::Grey => 48,
            Self::Taupe => 49,
        }
    }

    /// Returns the color's RGB values as a `u8` tuple.
    #[must_use]
    pub const fn rgb(&self) -> (u8, u8, u8) {
        match self {
            Self::BerryRed => (0xb8, 0x25, 0x5f),
            Self::Red => (0xdc, 0x4c, 0x3e),
            Self::Orange => (0xc7, 0x71, 0x00),
            Self::Yellow => (0xb2, 0x91, 0x04),
            Self::OliveGreen => (0x94, 0x9c, 0x31),
            Self::LimeGreen => (0x65, 0xa3, 0x3a),
            Self::Green => (0x36, 0x93, 0x07),
            Self::MintGreen => (0x42, 0xa3, 0x93),
            Self::Teal => (0x14, 0x8f, 0xad),
            Self::SkyBlue => (0x31, 0x9d, 0xc0),
            Self::LightBlue => (0x69, 0x88, 0xa4),
            Self::Blue => (0x41, 0x80, 0xff),
            Self::Grape => (0x69, 0x2e, 0xc2),
            Self::Violet => (0xca, 0x3f, 0xee),
            Self::Lavender => (0xa4, 0x69, 0x8c),
            Self::Magenta => (0xe0, 0x50, 0x95),
            Self::Salmon => (0xc9, 0x76, 0x6f),
            Self::Charcoal => (0x80, 0x80, 0x80),
            Self::Grey => (0x99, 0x99, 0x99),
            Self::Taupe => (0x8f, 0x7a, 0x69),
        }
    }

    /// Returns the color's hex string.
    #[must_use]
    pub fn hex(&self) -> String {
        let (r, g, b) = self.rgb();
        format!("#{r:02x}{g:02x}{b:02x}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // https://developer.todoist.com/api/v1/#tag/Colors
    mod color {
        use super::*;

        const ALL: &[Color] = &[
            Color::BerryRed,
            Color::Red,
            Color::Orange,
            Color::Yellow,
            Color::OliveGreen,
            Color::LimeGreen,
            Color::Green,
            Color::MintGreen,
            Color::Teal,
            Color::SkyBlue,
            Color::LightBlue,
            Color::Blue,
            Color::Grape,
            Color::Violet,
            Color::Lavender,
            Color::Magenta,
            Color::Salmon,
            Color::Charcoal,
            Color::Grey,
            Color::Taupe,
        ];

        #[test]
        fn all_colors_present() {
            // If new variant is ever added
            assert_eq!(
                ALL.len(),
                20,
                "update `ALL` when adding/removing a Color variant"
            );
        }

        #[test]
        fn ids_match_todoist_docs() {
            let expected: &[(Color, u8)] = &[
                (Color::BerryRed, 30),
                (Color::Red, 31),
                (Color::Orange, 32),
                (Color::Yellow, 33),
                (Color::OliveGreen, 34),
                (Color::LimeGreen, 35),
                (Color::Green, 36),
                (Color::MintGreen, 37),
                (Color::Teal, 38),
                (Color::SkyBlue, 39),
                (Color::LightBlue, 40),
                (Color::Blue, 41),
                (Color::Grape, 42),
                (Color::Violet, 43),
                (Color::Lavender, 44),
                (Color::Magenta, 45),
                (Color::Salmon, 46),
                (Color::Charcoal, 47),
                (Color::Grey, 48),
                (Color::Taupe, 49),
            ];
            for (color, id) in expected {
                assert_eq!(&color.id(), id, "{color:?} id mismatch");
            }
        }

        #[test]
        fn hex_matches_todoist_docs() {
            let expected: &[(Color, &str)] = &[
                (Color::BerryRed, "#b8255f"),
                (Color::Red, "#dc4c3e"),
                (Color::Orange, "#c77100"),
                (Color::Yellow, "#b29104"),
                (Color::OliveGreen, "#949c31"),
                (Color::LimeGreen, "#65a33a"),
                (Color::Green, "#369307"),
                (Color::MintGreen, "#42a393"),
                (Color::Teal, "#148fad"),
                (Color::SkyBlue, "#319dc0"),
                (Color::LightBlue, "#6988a4"),
                (Color::Blue, "#4180ff"),
                (Color::Grape, "#692ec2"),
                (Color::Violet, "#ca3fee"),
                (Color::Lavender, "#a4698c"),
                (Color::Magenta, "#e05095"),
                (Color::Salmon, "#c9766f"),
                (Color::Charcoal, "#808080"),
                (Color::Grey, "#999999"),
                (Color::Taupe, "#8f7a69"),
            ];
            for (color, hex) in expected {
                assert_eq!(&color.hex().to_lowercase(), hex, "{color:?} hex mismatch");
            }
        }
    }
}
