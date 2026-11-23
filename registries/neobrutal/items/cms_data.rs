#[derive(Clone, PartialEq)]
pub(super) struct Post {
    pub title: String,
    pub summary: String,
    pub body: String,
    pub published: bool,
}
pub(super) fn posts() -> Vec<Post> {
    vec![
    Post {title: "July product update".into(), summary: "Highlights from the latest workspace release.".into(), body: "This release brings clearer ownership, faster search, and a simpler review queue.\n\nStart with the updated workspace overview, then review the changes with your team.".into(), published: true},
    Post {title: "Organize your first team space".into(), summary: "A practical structure for growing teams.".into(), body: "Start with one shared space. Assign an owner to each area, write down the naming rules, and invite a small group to test the workflow.".into(), published: false},
    Post {title: "A faster review process".into(), summary: "How Verdant Studio simplified editorial review.".into(), body: "The team replaced scattered feedback with a single review queue. Every draft now has a named reviewer and a clear next action.".into(), published: true},
    Post {title: "Public page checklist".into(), summary: "Final ownership, access, and link checks.".into(), body: "Check ownership and access. Read the page on a narrow screen. Test every link and confirm that the contact details are current.".into(), published: false},
    Post {title: "Shared inbox routing".into(), summary: "Set routing rules and keep requests visible.".into(), body: "Route each request to one responsible team. Keep unassigned requests visible and review routing rules when the team changes.".into(), published: true},
    ]
}
