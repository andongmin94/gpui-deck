#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Status {
    Draft,
    Active,
    Archived,
}
impl Status {
    pub(crate) const ALL: [Self; 3] = [Self::Draft, Self::Active, Self::Archived];
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Draft => "Draft",
            Self::Active => "Active",
            Self::Archived => "Archived",
        }
    }
}
#[derive(Clone, Eq, PartialEq)]
pub(crate) struct Record {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) owner: String,
    pub(crate) status: Status,
    pub(crate) date: String,
}
pub(super) fn records() -> Vec<Record> {
    vec![
        Record {
            id: "REC-101".into(),
            title: "Website accessibility review".into(),
            owner: "Mina".into(),
            status: Status::Active,
            date: "2026-10-02".into(),
        },
        Record {
            id: "REC-102".into(),
            title: "Profile settings".into(),
            owner: "Alex".into(),
            status: Status::Draft,
            date: "2026-10-05".into(),
        },
        Record {
            id: "REC-103".into(),
            title: "Customer onboarding".into(),
            owner: "Jun".into(),
            status: Status::Active,
            date: "2026-10-08".into(),
        },
        Record {
            id: "REC-104".into(),
            title: "Notification preferences".into(),
            owner: "Alex".into(),
            status: Status::Draft,
            date: "2026-10-09".into(),
        },
        Record {
            id: "REC-105".into(),
            title: "Monthly product report".into(),
            owner: "Mina".into(),
            status: Status::Active,
            date: "2026-10-12".into(),
        },
        Record {
            id: "REC-106".into(),
            title: "Legacy welcome screen".into(),
            owner: "Jun".into(),
            status: Status::Archived,
            date: "2026-09-01".into(),
        },
        Record {
            id: "REC-107".into(),
            title: "Keyboard navigation audit".into(),
            owner: "Jun".into(),
            status: Status::Active,
            date: "2026-10-14".into(),
        },
        Record {
            id: "REC-108".into(),
            title: "Content planning".into(),
            owner: "Mina".into(),
            status: Status::Draft,
            date: "2026-10-15".into(),
        },
        Record {
            id: "REC-109".into(),
            title: "Release notes".into(),
            owner: "Alex".into(),
            status: Status::Active,
            date: "2026-10-20".into(),
        },
        Record {
            id: "REC-110".into(),
            title: "Archived campaign".into(),
            owner: "Mina".into(),
            status: Status::Archived,
            date: "2026-08-20".into(),
        },
        Record {
            id: "REC-111".into(),
            title: "Mobile layout polish".into(),
            owner: "Jun".into(),
            status: Status::Draft,
            date: "2026-10-21".into(),
        },
        Record {
            id: "REC-112".into(),
            title: "Support documentation".into(),
            owner: "Alex".into(),
            status: Status::Active,
            date: "2026-10-22".into(),
        },
    ]
}
