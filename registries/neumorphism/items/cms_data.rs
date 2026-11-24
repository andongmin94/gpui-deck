#[derive(Clone, PartialEq)]
pub(crate) struct Post {
    pub title: String,
    pub summary: String,
    pub body: String,
    pub published: bool,
}
impl Post {
    pub(super) fn untitled() -> Self {
        Self {
            title: "Untitled post".into(),
            summary: String::new(),
            body: String::new(),
            published: false,
        }
    }
}
pub(super) fn posts() -> Vec<Post> {
    vec![
Post {title:"September editor update".into(),summary:"Draft recovery and the post review checklist.".into(),body:"The editor keeps the current draft when a save fails. Retry the request without re-entering the title or body.".into(),published:true},
Post {title:"Prepare a post for review".into(),summary:"Check the title, summary and publication status.".into(),body:"Confirm the title matches the article. Write a summary that names its topic. Keep the status as Draft until the reviewer approves publication.".into(),published:false},
Post {title:"Assign a reviewer to each draft".into(),summary:"Record the reviewer and the outstanding changes.".into(),body:"Choose one reviewer for the draft. Collect requested changes in the article notes and confirm them before publishing.".into(),published:true},
]
}
