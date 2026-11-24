pub(super) struct Link {
    pub title: &'static str,
    pub detail: &'static str,
    pub group: &'static str,
    pub href: &'static str,
}
pub(super) const LINKS: &[Link] = &[
    Link {
        title: "Documentation",
        group: "Guides",
        href: "https://neumorphism-ui.andongmin.com/en/docs",
        detail: "neumorphism-ui.andongmin.com/en/docs",
    },
    Link {
        title: "Components",
        group: "Guides",
        href: "https://neumorphism-ui.andongmin.com/en/components",
        detail: "neumorphism-ui.andongmin.com/en/components",
    },
    Link {
        title: "GitHub",
        group: "Code",
        href: "https://github.com/andongmin94/neumorphism-ui",
        detail: "github.com/andongmin94/neumorphism-ui",
    },
    Link {
        title: "Report an issue",
        group: "Code",
        href: "https://github.com/andongmin94/neumorphism-ui/issues",
        detail: "github.com/andongmin94/neumorphism-ui/issues",
    },
    Link {
        title: "Email",
        group: "Contact",
        href: "mailto:hello@example.com",
        detail: "hello@example.com",
    },
];
