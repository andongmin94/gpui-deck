pub(crate) struct Article {
    pub title: &'static str,
    pub topic: &'static str,
    pub date: &'static str,
    pub summary: &'static str,
    pub duration: &'static str,
    pub minutes: u32,
    pub intro: &'static str,
    pub sections: &'static [(&'static str, &'static [&'static str])],
}
pub(crate) const ARTICLES: &[Article] = &[
    Article {
        title: "Recover a draft after a failed save",
        topic: "Product",
        date: "2026-09-18",
        summary: "Retain the edited title and body, report the error, and retry the save.",
        duration: "4 min read",
        minutes: 4,
        intro: "A failed save should not erase the title, summary or body that someone just edited. Keep the draft in the form and show the error next to the save action.",
        sections: &[
            (
                "Separate the draft from the saved copy",
                &[
                    "When the editor opens, start with the saved post and keep a separate draft for changes. Typing updates the draft; it does not mark the post as saved.",
                    "Only replace the saved copy after the save callback resolves. When it rejects, leave both the draft and the previous saved copy intact.",
                ],
            ),
            (
                "Retry without re-entering the post",
                &[
                    "After a failed request, the author should be able to correct a field or retry with the same draft.",
                    "• Show a save error without clearing the inputs.",
                    "• Keep Save available after the request finishes.",
                    "• Ask before discarding a draft with unsaved changes.",
                ],
            ),
        ],
    },
    Article {
        title: "Return keyboard focus after closing a dialog",
        topic: "Engineering",
        date: "2026-09-10",
        summary: "Check Escape, the close action and focus returning to the button that opened the dialog.",
        duration: "6 min read",
        minutes: 6,
        intro: "Closing a dialog should leave the keyboard user at a useful place. Check that focus returns to the button that opened it, so the next action is easy to reach.",
        sections: &[(
            "Check every close action",
            &[
                "Use the dialog primitive’s focus handling. Test the close button and Escape, then continue with Tab to check the order of the surrounding controls.",
                "• Open the dialog with the keyboard.",
                "• Close with Escape and check that the opening button receives focus.",
                "• Reopen it, use the close button, and check the same return path.",
            ],
        )],
    },
    Article {
        title: "Align icons in compact controls",
        topic: "Design",
        date: "2026-08-29",
        summary: "Use the same icon box for small buttons, menu indicators and disclosure controls.",
        duration: "5 min read",
        minutes: 5,
        intro: "Small controls are easier to scan when their icons use a consistent size and alignment. A shared icon box keeps buttons, menus and disclosure controls visually related.",
        sections: &[(
            "Align the icon box",
            &[
                "Give each icon the same width and height, keep it from shrinking, and center it beside the text. Check the result with both short and wrapping labels.",
                "• Use one icon size for controls with the same visual weight.",
                "• Keep the gap between the icon and label consistent.",
                "• Give icon-only buttons an accessible name; hide decorative icons from assistive technology.",
            ],
        )],
    },
];
