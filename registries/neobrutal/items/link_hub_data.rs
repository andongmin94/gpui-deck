pub(super) struct Link {
    pub title: &'static str,
    pub detail: &'static str,
    pub group: &'static str,
    pub href: &'static str,
}
pub(super) const LINKS: &[Link] = &[
    Link {
        title: "Portfolio",
        detail: "example.com",
        group: "Work",
        href: "https://example.com",
    },
    Link {
        title: "GitHub",
        detail: "github.com",
        group: "Work",
        href: "https://github.com",
    },
    Link {
        title: "X",
        detail: "x.com",
        group: "Connect",
        href: "https://x.com",
    },
    Link {
        title: "Newsletter",
        detail: "buttondown.email",
        group: "Writing",
        href: "https://buttondown.email",
    },
    Link {
        title: "LinkedIn",
        detail: "linkedin.com",
        group: "Connect",
        href: "https://www.linkedin.com",
    },
    Link {
        title: "Email",
        detail: "hello@example.com",
        group: "Connect",
        href: "mailto:hello@example.com",
    },
];
