use core::fmt;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GameDifficulty {
    #[default]
    Easy,
    Medium,
    Hard,
    Expert,
    Extreme,
}

impl GameDifficulty {
    pub const ALL: [Self; 5] = [
        Self::Easy,
        Self::Medium,
        Self::Hard,
        Self::Expert,
        Self::Extreme,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Easy => "Easy",
            Self::Medium => "Medium",
            Self::Hard => "Hard",
            Self::Expert => "Expert",
            Self::Extreme => "Extreme",
        }
    }

    pub const fn slug(self) -> &'static str {
        match self {
            Self::Easy => "easy",
            Self::Medium => "medium",
            Self::Hard => "hard",
            Self::Expert => "expert",
            Self::Extreme => "extreme",
        }
    }

    pub const fn target_clues(self) -> usize {
        match self {
            Self::Easy => 40,
            Self::Medium => 32,
            Self::Hard => 28,
            Self::Expert => 24,
            Self::Extreme => 22,
        }
    }

    pub fn from_slug(slug: &str) -> Option<Self> {
        match slug {
            "easy" => Some(Self::Easy),
            "medium" => Some(Self::Medium),
            "hard" => Some(Self::Hard),
            "expert" => Some(Self::Expert),
            "extreme" => Some(Self::Extreme),
            _ => None,
        }
    }
}

impl fmt::Display for GameDifficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}