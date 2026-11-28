use gpui::{Context, Entity, FocusHandle, Subscription, Window, div, prelude::*, rgb};
use gpui_deck::{
    Button, Dialog, DialogBackdrop, DialogClose, DialogPopup, DialogState, DialogTitle, Root,
};

struct ProfileDialog {
    dialog: Entity<DialogState>,
    close_focus: FocusHandle,
    _observation: Subscription,
}

impl ProfileDialog {
    fn new(cx: &mut Context<Self>) -> Self {
        let close_focus = cx.focus_handle();
        let dialog = cx.new(|cx| DialogState::new(cx).with_initial_focus(&close_focus));
        let observation = cx.observe(&dialog, |_, _, cx| cx.notify());
        Self {
            dialog,
            close_focus,
            _observation: observation,
        }
    }
}

impl Render for ProfileDialog {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let title = "Profile details";
        let open = Button::new("open-profile")
            .accessibility_label(title)
            .px_3()
            .py_2()
            .rounded_md()
            .border_1()
            .focus_visible(|style| style.bg(rgb(0xe4e4e7)))
            .child(title)
            .on_click(cx.listener(|this, _, window, cx| {
                this.dialog.update(cx, |state, cx| state.open(window, cx));
            }));
        let popup = DialogPopup::new()
            .flex()
            .flex_col()
            .gap_3()
            .p_5()
            .rounded_md()
            .bg(rgb(0xffffff))
            .child(DialogTitle::new().child(title))
            .child("Signed in as Ada Park")
            .child(DialogClose::new(&self.dialog).trigger(|button| {
                button
                    .track_focus(&self.close_focus)
                    .px_3()
                    .py_2()
                    .border_1()
                    .rounded_md()
                    .focus_visible(|style| style.bg(rgb(0xe4e4e7)))
                    .child("Close")
            }));

        div()
            .size_full()
            .p_6()
            .bg(rgb(0xffffff))
            .text_color(rgb(0x18181b))
            .child(open)
            .child(
                Dialog::new(&self.dialog)
                    .accessibility_label(title)
                    .backdrop(
                        DialogBackdrop::new()
                            .size_full()
                            .bg(gpui::black().opacity(0.4)),
                    )
                    .popup(popup),
            )
    }
}

fn main() {
    gpui_platform::application().run(|cx| {
        gpui_deck::init(cx);
        cx.on_window_closed(|cx, _| cx.quit()).detach();
        cx.open_window(Default::default(), |window, cx| {
            let screen = cx.new(ProfileDialog::new);
            cx.new(|cx| Root::new(screen, window, cx))
        })
        .expect("open window");
        cx.activate(true);
    });
}
