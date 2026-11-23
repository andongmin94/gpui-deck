//! Editable archive and reading pages from neobrutal-ui.
pub(super) struct Article {
    pub title: &'static str,
    pub topic: &'static str,
    pub date: &'static str,
    pub duration: &'static str,
    pub summary: &'static str,
    pub intro: &'static str,
    pub sections: &'static [(&'static str, &'static [&'static str])],
}

pub(super) const ARTICLES: &[Article] = &[
    Article {
        title: "Why Small Interfaces Age Better",
        topic: "Design",
        date: "2026-07-12",
        duration: "5 min",
        summary: "A practical case for fewer controls, clearer defaults, and less maintenance.",
        intro: "Small interfaces are easier to understand on the first visit and easier to maintain on the hundredth release.",
        sections: &[
            (
                "Start with the boundary",
                &[
                    "A useful interface begins by deciding what it will not ask the user to manage. Every exposed option becomes a promise the product has to explain, support, and preserve.",
                    "Keep the controls that change the outcome. Move rare configuration closer to the moment it becomes relevant instead of presenting it up front.",
                ],
            ),
            (
                "Make the default path obvious",
                &[
                    "A strong default removes a decision without hiding what happened. People should be able to complete the common task and still understand the system's choice.",
                    "• Name the primary action precisely.",
                    "• Show the current state beside the control that changes it.",
                    "• Keep advanced choices available without making them mandatory.",
                ],
            ),
            (
                "Leave room to remove things",
                &[
                    "Maintenance gets lighter when each control has a clear owner and measurable purpose. If nobody can explain why an option still exists, removing it may be the most useful design improvement.",
                ],
            ),
        ],
    },
    Article {
        title: "Reading a Codebase Before Changing It",
        topic: "Engineering",
        date: "2026-07-06",
        duration: "8 min",
        summary: "The files and questions that reveal how an unfamiliar project really works.",
        intro: "The fastest way into an unfamiliar codebase is to follow one real workflow from its entry point to its visible result.",
        sections: &[
            (
                "Trace one path end to end",
                &[
                    "Begin with a route, command, or event that you can reproduce. Follow the data through the components and services it touches, and note where validation and side effects happen.",
                    "This gives the directory structure a purpose. Names that looked abstract become parts of a concrete flow.",
                ],
            ),
            (
                "Find the contracts",
                &[
                    "Types, tests, schemas, and adapters describe the boundaries that other code already depends on. Read those before changing an implementation detail that appears local.",
                    "• Inputs accepted at the boundary",
                    "• Outputs consumed by another module",
                    "• Errors that are intentionally surfaced",
                    "• Tests that describe behavior rather than implementation",
                ],
            ),
            (
                "Write a small change map",
                &[
                    "Before editing, list the files that own the behavior, the callers that rely on it, and the checks that will prove the change. The map can be short; its job is to prevent a local fix from becoming a hidden system change.",
                ],
            ),
        ],
    },
    Article {
        title: "Empty States Should Point Somewhere",
        topic: "Product",
        date: "2026-06-28",
        duration: "4 min",
        summary: "Five ways to turn a blank screen into a useful next step.",
        intro: "An empty state is not a pause in the product. It is the moment when the interface has the most responsibility to explain what comes next.",
        sections: &[
            (
                "Explain why it is empty",
                &[
                    "No data can mean a first visit, a narrow filter, a failed request, or missing access. Use language that reflects the actual cause instead of showing the same generic message everywhere.",
                ],
            ),
            (
                "Offer one useful next step",
                &[
                    "The primary action should resolve the empty state or help the user understand it. Secondary links can provide examples or documentation, but they should not compete with the next action.",
                    "• Create the first item",
                    "• Clear the active filters",
                    "• Invite a teammate who owns the data",
                    "• Retry a failed request",
                    "• Open a relevant example",
                ],
            ),
            (
                "Keep the state compact",
                &[
                    "A blank screen does not need a marketing page inside it. A direct heading, one sentence, and a clear action usually provide enough context without slowing down repeat users.",
                ],
            ),
        ],
    },
    Article {
        title: "A Quieter Way to Debug Frontend Work",
        topic: "Engineering",
        date: "2026-06-19",
        duration: "7 min",
        summary: "A short loop for isolating layout, state, and network problems without guessing.",
        intro: "Debugging gets faster when each experiment answers one question and changes one variable.",
        sections: &[
            (
                "Reduce the surface area",
                &[
                    "Reproduce the problem in the smallest route, viewport, and state you can. Disable unrelated animation and data refreshes so the signal does not move while you inspect it.",
                ],
            ),
            (
                "Use a short loop",
                &[
                    "State the current hypothesis before touching the code. Then run the smallest observation or change that could disprove it.",
                    "• Observe the DOM, computed styles, state, and request timing.",
                    "• Change one variable.",
                    "• Reproduce from the same starting state.",
                    "• Keep or revert the change immediately.",
                ],
            ),
            (
                "Record the useful result",
                &[
                    "When the cause is subtle, leave a focused regression test or a short comment near the constraint. The goal is not to preserve the debugging story; it is to prevent the same ambiguity from returning.",
                ],
            ),
        ],
    },
    Article {
        title: "Write Down the Decision, Not the Meeting",
        topic: "Workflow",
        date: "2026-06-11",
        duration: "6 min",
        summary: "A tiny note format that preserves context without becoming more process.",
        intro: "A useful decision note lets someone understand what changed and why without replaying the meeting that produced it.",
        sections: &[
            (
                "Capture the durable parts",
                &[
                    "Attendance, discussion order, and every rejected sentence rarely matter later. The decision, the constraints behind it, and the consequences for future work usually do.",
                    "• Decision: what the team will do",
                    "• Context: the constraint or evidence that shaped it",
                    "• Consequences: what becomes easier, harder, or intentionally unsupported",
                ],
            ),
            (
                "Keep one source of truth",
                &[
                    "Link the note from the issue, pull request, or project page where the work continues. Copies in several tools drift quickly and make a settled decision look unresolved.",
                ],
            ),
            (
                "Update decisions explicitly",
                &[
                    "A decision can change. Add a short superseding note and link the two records instead of silently rewriting the old context. Future readers can then see both the current direction and the reason it moved.",
                ],
            ),
        ],
    },
    Article {
        title: "Shorter Forms Without Hidden Questions",
        topic: "Design",
        date: "2026-06-02",
        duration: "5 min",
        summary: "Use grouping, timing, and defaults to reduce friction while staying honest.",
        intro: "A form feels short when every question arrives at the right moment and its purpose is easy to understand.",
        sections: &[
            (
                "Ask only when the answer matters",
                &[
                    "Collect information at the point where it changes the result. Questions needed only for a later workflow can wait until that workflow begins.",
                ],
            ),
            (
                "Use defaults without hiding choices",
                &[
                    "A default should reflect the most common safe answer and remain visible to the person submitting the form. Preselected values that are easy to miss save clicks by creating uncertainty elsewhere.",
                    "• Group fields by the task they support.",
                    "• Explain why sensitive information is required.",
                    "• Reveal conditional questions as soon as the triggering choice is made.",
                ],
            ),
            (
                "Audit the work after submit",
                &[
                    "Reducing visible fields is not an improvement if the user must correct assumptions on the next screen. Review the entire path through confirmation and editing, not only the initial form length.",
                ],
            ),
        ],
    },
];
