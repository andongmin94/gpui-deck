//! Copyable Rust sources, dependencies, and assets used by the GPUI Deck CLI.

/// One recipe and the files and modules needed to install it in an application.
pub struct Recipe {
    pub name: &'static str,
    pub module: &'static str,
    pub exports: &'static [&'static str],
    pub dependencies: &'static [&'static str],
    pub source: &'static str,
    pub assets: &'static [(&'static str, &'static str)],
}

macro_rules! recipe {
    ($name:literal, $module:ident, [$($export:ident),+], [$($dependency:literal),*] $(, $asset:expr)?) => {
        Recipe {
            name: $name,
            module: stringify!($module),
            exports: &[$(stringify!($export)),+],
            dependencies: &[$($dependency),*],
            source: include_str!(concat!("../examples/recipes/", stringify!($module), ".rs")),
            assets: &[$($asset)?],
        }
    };
}

// Names, exports, source dependencies and assets have one owner. Source itself
// stays in the examples; rebuilding the CLI embeds those exact current files.
/// All copyable recipes in this release.
pub const RECIPES: &[Recipe] = &[
    recipe!("alert", alert, [Alert], []),
    recipe!("aspect-ratio", aspect_ratio, [AspectRatio], []),
    recipe!("attachment", attachment, [Attachment], []),
    recipe!(
        "avatar",
        avatar,
        [Avatar, AvatarGroup],
        [],
        (
            "assets/preset-avatar.svg",
            include_str!("../examples/assets/preset-avatar.svg")
        )
    ),
    recipe!("badge", badge, [Badge], []),
    recipe!("bubble", bubble, [Bubble, BubbleSide], []),
    recipe!("button", button, [ButtonVariant], []),
    recipe!("button-group", button_group, [ButtonGroup], []),
    recipe!("card", card, [Card], []),
    recipe!("chart", chart, [ChartKind, ChartRecipe], []),
    recipe!("code-block", code_block, [CodeBlock], []),
    recipe!(
        "data-table",
        data_table,
        [
            DataTable,
            DataTableCell,
            DataTableColumn,
            DataTableRow,
            DataTableState
        ],
        ["empty", "input-group", "pagination"]
    ),
    recipe!("empty", empty, [EmptyState], []),
    recipe!("input-group", input_group, [InputGroup], []),
    recipe!("item", item, [Item], []),
    recipe!("kbd", kbd, [Kbd, KbdGroup], []),
    recipe!("marker", marker, [Marker], []),
    recipe!("message", message, [Message], ["typography"]),
    recipe!("pagination", pagination, [Pagination], []),
    recipe!("sheet", sheet, [Sheet, SheetSide], []),
    recipe!("sidebar", sidebar, [Sidebar], ["sheet"]),
    recipe!("skeleton", skeleton, [Skeleton], []),
    recipe!("spinner", spinner, [Spinner], []),
    recipe!("trace", trace, [Trace], ["typography"]),
    recipe!(
        "typography",
        typography,
        [Heading, InlineCode, Paragraph],
        []
    ),
];

/// Application-owned palette and typography copied with every installation.
pub const PRESENTATION: &str = include_str!("../examples/presentation.rs");
/// Additional faces used by the Create gallery's field labels and headings.
pub const PRETENDARD_MEDIUM: &[u8] =
    include_bytes!("../examples/assets/fonts/Pretendard-Medium.otf");
pub const PRETENDARD_SEMIBOLD: &[u8] =
    include_bytes!("../examples/assets/fonts/Pretendard-SemiBold.otf");
/// License distributed with the recipe sources.
pub const LICENSE: &str = include_str!("../LICENSE-APACHE");
