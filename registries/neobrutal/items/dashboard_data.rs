pub(super) struct Task {
    pub title: &'static str,
    pub project: &'static str,
    pub owner: &'static str,
    pub done: bool,
}
pub(super) fn tasks() -> Vec<Task> {
    vec![
        Task {
            title: "Finalize the product brief",
            project: "Website refresh",
            owner: "You",
            done: true,
        },
        Task {
            title: "Build the core pages",
            project: "Website refresh",
            owner: "You",
            done: false,
        },
        Task {
            title: "Review launch content",
            project: "Website refresh",
            owner: "Maya",
            done: false,
        },
        Task {
            title: "Audit existing components",
            project: "Design system",
            owner: "Leo",
            done: true,
        },
        Task {
            title: "Document form patterns",
            project: "Design system",
            owner: "You",
            done: false,
        },
        Task {
            title: "Complete customer interviews",
            project: "Customer research",
            owner: "Maya",
            done: true,
        },
        Task {
            title: "Share research findings",
            project: "Customer research",
            owner: "Leo",
            done: false,
        },
    ]
}
