use gpui::{Context, Entity, Subscription, Window, div, prelude::*, px, rgb};
use gpui_deck::{Combobox, ComboboxList, ComboboxOption, ComboboxState, Root, SearchItem};

struct FrameworkPicker {
    frameworks: Entity<ComboboxState>,
    _observation: Subscription,
}

impl FrameworkPicker {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let frameworks = cx.new(|cx| {
            ComboboxState::new(
                [
                    SearchItem::new("next", "Next.js").keywords(["react"]),
                    SearchItem::new("sveltekit", "SvelteKit"),
                    SearchItem::new("astro", "Astro").disabled(true),
                ],
                window,
                cx,
            )
        });
        let observation = cx.observe(&frameworks, |_, _, cx| cx.notify());
        Self {
            frameworks,
            _observation: observation,
        }
    }
}

impl Render for FrameworkPicker {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.frameworks.read(cx);
        let visible = state.visible_indices(cx);
        let options = visible.iter().map(|&index| {
            let item = &state.items(cx)[index];
            ComboboxOption::new(&self.frameworks, index)
                .h(px(32.))
                .px_3()
                .when(state.highlighted_index(cx) == Some(index), |row| {
                    row.bg(rgb(0xe4e4e7))
                })
                .when(item.is_disabled(), |row| row.opacity(0.45))
                .child(item.label().clone())
        });
        let list = ComboboxList::new("framework-list", &self.frameworks)
            .accessibility_label("Frameworks")
            .max_h(px(144.))
            .children(options);

        div()
            .size_full()
            .p_6()
            .bg(rgb(0xffffff))
            .text_color(rgb(0x18181b))
            .child(
                Combobox::new("framework", &self.frameworks)
                    .w(px(240.))
                    .input(|input| {
                        input
                            .accessibility_label("Framework")
                            .placeholder("Search frameworks")
                            .h(px(36.))
                            .px_3()
                            .border_1()
                            .rounded_md()
                            .styles(|styles| {
                                styles.focused(|style| style.border_color(rgb(0x2563eb)))
                            })
                    })
                    .content(
                        div()
                            .w(px(240.))
                            .bg(rgb(0xffffff))
                            .border_1()
                            .child(list)
                            .when(visible.is_empty(), |popup| popup.child("No matches")),
                    ),
            )
    }
}

fn main() {
    gpui_platform::application().run(|cx| {
        gpui_deck::init(cx);
        cx.on_window_closed(|cx, _| cx.quit()).detach();
        cx.open_window(Default::default(), |window, cx| {
            let screen = cx.new(|cx| FrameworkPicker::new(window, cx));
            cx.new(|cx| Root::new(screen, window, cx))
        })
        .expect("open window");
        cx.activate(true);
    });
}
