pub mod cell {
    pub const TEXT_GIVEN: &str = "text-indigo-700 dark:text-indigo-300 font-semibold";
    pub const TEXT_USER: &str = "text-zinc-900 dark:text-zinc-100 font-medium";
    pub const TEXT_EMPTY: &str = "text-zinc-400 dark:text-zinc-600";

    pub const BG_DEFAULT: &str = "bg-white dark:bg-zinc-950";
    pub const BG_PEER: &str = "bg-zinc-100 dark:bg-zinc-900";
    pub const BG_SAME_VALUE: &str = "bg-blue-50 dark:bg-blue-950/40";
    pub const BG_SELECTED: &str = "bg-blue-100 dark:bg-blue-950";
    pub const BG_CONFLICT: &str = "bg-red-100 dark:bg-red-950";
    pub const BG_HINT_PLACEMENT: &str = "bg-emerald-50 dark:bg-emerald-950/40";

    pub const BORDER_THIN: &str = "border-zinc-300 dark:border-zinc-800";
    pub const BORDER_THICK: &str = "border-zinc-400 dark:border-zinc-600";

    pub const CANDIDATE_VISIBLE: &str = "text-zinc-500 dark:text-zinc-400";
    pub const CANDIDATE_HIDDEN: &str = "text-transparent";
    pub const CANDIDATE_HINT: &str = "text-amber-600 dark:text-amber-400";
}

pub mod grid {
    pub const BORDER: &str = "border-zinc-400 dark:border-zinc-600";
}

pub mod status {
    pub const TEXT_PRIMARY: &str = "text-zinc-900 dark:text-zinc-100";
    pub const TEXT_SECONDARY: &str = "text-zinc-500 dark:text-zinc-500";
    pub const TEXT_MUTED: &str = "text-zinc-400 dark:text-zinc-600";
    pub const TEXT_ERROR: &str = "text-red-600 dark:text-red-400";
}

pub mod controls {
    pub const BUTTON_BASE: &str = "px-3 py-1.5 text-sm rounded border transition-colors";
    pub const BUTTON_DEFAULT: &str =
        "border-zinc-200 dark:border-zinc-800 hover:bg-zinc-100 dark:hover:bg-zinc-800";
    pub const BUTTON_ACTIVE: &str = "border-blue-500 text-blue-600 dark:text-blue-400";
    pub const BUTTON_SUCCESS: &str =
        "border-emerald-400 dark:border-emerald-700 text-emerald-700 dark:text-emerald-400 \
         hover:bg-emerald-50 dark:hover:bg-emerald-950";
    pub const BUTTON_DISABLED: &str = "disabled:opacity-40 disabled:cursor-not-allowed";
}

pub mod chrome {
    pub const BACKGROUND: &str = "bg-white dark:bg-zinc-950";
    pub const SURFACE: &str = "bg-white/80 dark:bg-zinc-950/80 backdrop-blur";
    pub const BORDER: &str = "border-zinc-200 dark:border-zinc-800";
    pub const TEXT_PRIMARY: &str = "text-zinc-900 dark:text-zinc-100";
    pub const TEXT_SECONDARY: &str = "text-zinc-600 dark:text-zinc-400";
    pub const TEXT_MUTED: &str = "text-zinc-500 dark:text-zinc-500";
}

pub mod overlay {
    pub const BACKDROP: &str = "bg-black/50 dark:bg-black/70";
    pub const CARD: &str =
        "bg-white dark:bg-zinc-900 rounded-lg shadow-xl border border-zinc-200 dark:border-zinc-800";
    pub const TITLE: &str = "text-xl font-semibold text-zinc-900 dark:text-zinc-100";
    pub const TEXT: &str = "text-sm text-zinc-600 dark:text-zinc-400";
    pub const BUTTON_PRIMARY: &str =
        "w-full py-2.5 rounded-md bg-zinc-900 dark:bg-zinc-100 text-white dark:text-zinc-900 \
         text-sm font-medium hover:bg-zinc-700 dark:hover:bg-zinc-300 transition-colors";
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellVisual {
    Selected,
    Conflict,
    HintPlacement,
    SameValue,
    Peer,
    Default,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellText {
    Empty,
    Given,
    User,
}

pub fn cell_background(visual: CellVisual) -> &'static str {
    match visual {
        CellVisual::Selected => cell::BG_SELECTED,
        CellVisual::Conflict => cell::BG_CONFLICT,
        CellVisual::HintPlacement => cell::BG_HINT_PLACEMENT,
        CellVisual::SameValue => cell::BG_SAME_VALUE,
        CellVisual::Peer => cell::BG_PEER,
        CellVisual::Default => cell::BG_DEFAULT,
    }
}

pub fn cell_text(kind: CellText) -> &'static str {
    match kind {
        CellText::Empty => cell::TEXT_EMPTY,
        CellText::Given => cell::TEXT_GIVEN,
        CellText::User => cell::TEXT_USER,
    }
}

pub fn cell_border(row: u8, col: u8) -> &'static str {
    let thick_right = col == 2 || col == 5;
    let thick_bottom = row == 2 || row == 5;
    let last_col = col == 8;
    let last_row = row == 8;

    match (thick_right, thick_bottom, last_col, last_row) {
        (_, _, true, true) => "",
        (_, _, true, _) if thick_bottom => {
            "border-b-2 border-zinc-400 dark:border-zinc-600"
        }
        (_, _, true, _) => "border-b border-zinc-300 dark:border-zinc-800",
        (_, _, _, true) if thick_right => {
            "border-r-2 border-zinc-400 dark:border-zinc-600"
        }
        (_, _, _, true) => "border-r border-zinc-300 dark:border-zinc-800",
        (true, true, _, _) => "border-r-2 border-b-2 border-zinc-400 dark:border-zinc-600",
        (true, false, _, _) => "border-r-2 border-b border-zinc-400 dark:border-zinc-600",
        (false, true, _, _) => "border-r border-b-2 border-zinc-400 dark:border-zinc-600",
        (false, false, _, _) => "border-r border-b border-zinc-300 dark:border-zinc-800",
    }
}