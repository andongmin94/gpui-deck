use gpui::{AnyView, App, AppContext as _, Window};

pub(crate) fn mount(window: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|cx| crate::ui::billing_dashboard::BillingDashboard::new(window, cx))
        .into()
}

pub(crate) const TITLE: &str = "Billing dashboard";
pub(crate) const WINDOW_SIZE: (f32, f32) = (1176., 806.);
