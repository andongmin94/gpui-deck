//! Form, selection, date and data-display demos.

use gpui::{
    AnyView, App, AppContext as _, Axis, Bounds, Context, Div, ElementId, Entity, FocusHandle,
    Focusable as _, FontWeight, Hsla, InteractiveElement as _, IntoElement, ParentElement as _,
    Render, Role, ScrollHandle, SharedString, StatefulInteractiveElement as _, Styled as _,
    Subscription, Window, accesskit::SortDirection, canvas, div, fill, hsla, point,
    prelude::FluentBuilder as _, px, size,
};
use gpui_design::{
    Button, Calendar, CalendarDay, CalendarSelection, CalendarState, Carousel, CarouselControl,
    CarouselSlide, CarouselState, Chart, ChartSeries, ChartState, Combobox, ComboboxList,
    ComboboxOption, ComboboxState, DatePicker, DatePickerState, Direction, DirectionProvider,
    Field, Input, InputAccessibility, InputBase, InputEditorStyle, InputOtp, InputOtpEvent,
    InputOtpState, InputState, Label, MessageScroller, MessageScrollerState, NaiveDate,
    Questionnaire, QuestionnaireStage, QuestionnaireState, Resizable, ResizablePanel,
    ResizableState, ScrollArea, ScrollbarMode, SearchItem, Select, SelectItem, SelectOption,
    SelectState, Table, TableCaption, TableCell, TableColumn, TableRow, Textarea, TextareaState,
    Toggle, ToggleGroup, ToggleGroupState, ToggleItem, ToggleOption, input::InputEvent,
};

use super::{frame, outline, primary};
use crate::theme::Theme;

/// Demo names this module renders.
pub const NAMES: &[&str] = &[
    "field",
    "textarea",
    "label",
    "select",
    "combobox",
    "input-otp",
    "calendar",
    "date-picker",
    "table",
    "scroll-area",
    "resizable",
    "carousel",
    "chart",
    "direction",
    "message-scroller",
    "questionnaire",
];

pub fn open(name: &str, theme: Theme, window: &mut Window, cx: &mut App) -> Option<AnyView> {
    Some(match name {
        "field" => field(theme, window, cx),
        "textarea" => textarea(theme, window, cx),
        "label" => label(theme, window, cx),
        "select" => select(theme, cx),
        "combobox" => combobox(theme, window, cx),
        "input-otp" => input_otp(theme, window, cx),
        "calendar" => calendar(theme, cx),
        "date-picker" => date_picker(theme, window, cx),
        "table" => table(theme, cx),
        "scroll-area" => scroll_area(theme, cx),
        "resizable" => resizable(theme, cx),
        "carousel" => carousel(theme, cx),
        "chart" => chart(theme, cx),
        "direction" => direction(theme, cx),
        "message-scroller" => message_scroller(theme, cx),
        "questionnaire" => questionnaire(theme, window, cx),
        _ => return None,
    })
}

// Shared pieces

fn today() -> NaiveDate {
    NaiveDate::from_ymd_opt(2026, 11, 20).unwrap()
}

/// Readable editing colors for an input on the given theme.
fn editor_style(theme: Theme) -> InputEditorStyle {
    InputEditorStyle {
        foreground: theme.foreground,
        muted_foreground: theme.muted_foreground,
        caret: theme.foreground,
        selection: theme.ring.opacity(0.35),
        ..Default::default()
    }
}

fn text_input(
    theme: Theme,
    label: &str,
    placeholder: &str,
    window: &mut Window,
    cx: &mut App,
) -> Entity<InputState> {
    let label = SharedString::from(label.to_owned());
    let placeholder = SharedString::from(placeholder.to_owned());
    cx.new(|cx| {
        let mut state = InputState::new(window, cx).placeholder(placeholder.clone());
        state.set_editor_style(editor_style(theme));
        state.set_accessibility(
            InputAccessibility::new(Some(Role::TextInput))
                .with_label(Some(label))
                .with_placeholder(Some(placeholder)),
        );
        state
    })
}

/// The application-owned frame around a single-line input.
fn input_frame(
    theme: Theme,
    id: impl Into<ElementId>,
    input: &Entity<InputState>,
    invalid: bool,
    window: &Window,
    cx: &App,
) -> InputBase {
    let focused = input.focus_handle(cx).is_focused(window);
    InputBase::new(id)
        .role(None::<Role>)
        .semantic_owner::<Input>(input.entity_id())
        .focused(focused)
        .h(px(36.))
        .w_full()
        .px(px(12.))
        .rounded(px(6.))
        .border_1()
        .border_color(if invalid {
            theme.destructive
        } else if focused {
            theme.ring
        } else {
            theme.border
        })
        .bg(theme.background)
        .child(Input::new(input))
}

/// A popup surface: background, border, radius 8 and padding 4.
fn popup(theme: Theme) -> Div {
    div()
        .p(px(4.))
        .rounded(px(8.))
        .border_1()
        .border_color(theme.border)
        .bg(theme.background)
        .text_color(theme.foreground)
}

fn muted(theme: Theme, text: impl Into<SharedString>) -> Div {
    div().text_color(theme.muted_foreground).child(text.into())
}

fn heading(text: &'static str) -> Div {
    div().font_weight(FontWeight::SEMIBOLD).child(text)
}

// Field

struct FieldDemo {
    theme: Theme,
    input: Entity<InputState>,
    invalid: bool,
    saved: Option<SharedString>,
    _subscriptions: Vec<Subscription>,
}

impl FieldDemo {
    fn submit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let value = self.input.read(cx).value();
        if value.trim().is_empty() {
            self.invalid = true;
            self.saved = None;
            self.input.update(cx, |input, cx| input.focus(window, cx));
        } else {
            self.saved = Some(format!("Saved “{}”.", value.trim()).into());
        }
        cx.notify();
    }
}

impl Render for FieldDemo {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        frame(theme).child(
            div()
                .w(px(300.))
                .flex()
                .flex_col()
                .gap(px(16.))
                .child(
                    Field::new("username-field", &self.input, "Username")
                        .description("This is your public display name.")
                        .required(true)
                        .error(self.invalid.then(|| "Username is required.".into()))
                        .styles(|styles| {
                            styles
                                .label(|style| style.font_weight(FontWeight::MEDIUM))
                                .description(|style| style.text_color(theme.muted_foreground))
                                .error(|style| style.text_color(theme.destructive))
                        })
                        .flex()
                        .flex_col()
                        .gap(px(8.))
                        .child(input_frame(
                            theme,
                            "username",
                            &self.input,
                            self.invalid,
                            window,
                            cx,
                        )),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(12.))
                        .child(
                            primary(theme, Button::new("submit"))
                                .accessibility_label("Submit")
                                .child("Submit")
                                .on_click(
                                    cx.listener(|this, _, window, cx| this.submit(window, cx)),
                                ),
                        )
                        .when_some(self.saved.clone(), |row, saved| {
                            row.child(muted(theme, saved))
                        }),
                ),
        )
    }
}

fn field(theme: Theme, window: &mut Window, cx: &mut App) -> AnyView {
    let input = text_input(theme, "Username", "ada", window, cx);
    cx.new(|cx| {
        let subscriptions = vec![
            cx.observe(&input, |_, _, cx| cx.notify()),
            cx.subscribe_in(
                &input,
                window,
                |this: &mut FieldDemo, _, event, window, cx| match event {
                    InputEvent::Change => {
                        this.invalid = false;
                        this.saved = None;
                        cx.notify();
                    }
                    InputEvent::PressEnter { .. } => this.submit(window, cx),
                    _ => {}
                },
            ),
        ];
        FieldDemo {
            theme,
            input,
            invalid: false,
            saved: None,
            _subscriptions: subscriptions,
        }
    })
    .into()
}

// Textarea

struct TextareaDemo {
    theme: Theme,
    notes: Entity<TextareaState>,
    sent: bool,
    _observe: Subscription,
}

impl Render for TextareaDemo {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        let focused = self.notes.focus_handle(cx).is_focused(window);
        let value = self.notes.read(cx).value();
        let notes = self.notes.clone();
        frame(theme).child(
            div()
                .w(px(340.))
                .flex()
                .flex_col()
                .gap(px(12.))
                .child(
                    Field::new("message-field", &self.notes, "Your message")
                        .styles(|styles| {
                            styles.label(|style| style.font_weight(FontWeight::MEDIUM))
                        })
                        .flex()
                        .flex_col()
                        .gap(px(8.))
                        .child(
                            InputBase::new("message")
                                .role(None::<Role>)
                                .semantic_owner::<Textarea>(self.notes.entity_id())
                                .focused(focused)
                                .w_full()
                                .px(px(12.))
                                .py(px(8.))
                                .rounded(px(6.))
                                .border_1()
                                .border_color(if focused { theme.ring } else { theme.border })
                                .bg(theme.background)
                                .child(Textarea::new(&self.notes)),
                        ),
                )
                .child(muted(
                    theme,
                    if self.sent {
                        "Message sent.".to_string()
                    } else {
                        format!(
                            "{} characters · Enter adds a new line",
                            value.chars().count()
                        )
                    },
                ))
                .child(
                    primary(theme, Button::new("send"))
                        .accessibility_label("Send message")
                        .disabled(value.trim().is_empty())
                        .child("Send message")
                        .on_click(cx.listener(move |this, _, window, cx| {
                            notes.update(cx, |state, cx| state.set_value("", window, cx));
                            this.sent = true;
                            cx.notify();
                        })),
                ),
        )
    }
}

fn textarea(theme: Theme, window: &mut Window, cx: &mut App) -> AnyView {
    let notes = cx.new(|cx| {
        let mut state = TextareaState::new(window, cx)
            .rows(4)
            .placeholder("Type your message here.");
        state.set_editor_style(editor_style(theme));
        state.set_accessibility(
            InputAccessibility::new(Some(Role::MultilineTextInput))
                .with_placeholder(Some("Type your message here.".into())),
        );
        state
    });
    cx.new(|cx| {
        let observe = cx.observe(&notes, |this: &mut TextareaDemo, notes, cx| {
            if !notes.read(cx).value().is_empty() {
                this.sent = false;
            }
            cx.notify();
        });
        TextareaDemo {
            theme,
            notes,
            sent: false,
            _observe: observe,
        }
    })
    .into()
}

// Label

struct LabelDemo {
    theme: Theme,
    email: Entity<InputState>,
    _observe: Subscription,
}

impl Render for LabelDemo {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        frame(theme).child(
            div()
                .w(px(280.))
                .flex()
                .flex_col()
                .gap(px(8.))
                .child(
                    Label::new("email-label", &self.email.focus_handle(cx), "Email")
                        .font_weight(FontWeight::MEDIUM),
                )
                .child(input_frame(theme, "email", &self.email, false, window, cx))
                .child(muted(theme, "Click the label to focus the field.").text_size(px(13.))),
        )
    }
}

fn label(theme: Theme, window: &mut Window, cx: &mut App) -> AnyView {
    let email = text_input(theme, "Email", "m@example.com", window, cx);
    cx.new(|cx| {
        let observe = cx.observe(&email, |_, _, cx| cx.notify());
        LabelDemo {
            theme,
            email,
            _observe: observe,
        }
    })
    .into()
}

// Select

struct SelectDemo {
    theme: Theme,
    state: Entity<SelectState>,
    _observe: Subscription,
}

impl Render for SelectDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        let state = self.state.read(cx);
        let selected = state.selected_index();
        let highlighted = state.highlighted_index();
        let current = state
            .selected_item()
            .map(|item| item.label().clone())
            .unwrap_or_else(|| "Select a fruit".into());
        let has_value = selected.is_some();
        let options = state.items().iter().enumerate().map(|(index, item)| {
            SelectOption::new(&self.state, index)
                .h(px(32.))
                .px(px(8.))
                .flex()
                .items_center()
                .justify_between()
                .rounded(px(6.))
                .when(highlighted == Some(index), |option| option.bg(theme.accent))
                .when(item.is_disabled(), |option| option.opacity(0.5))
                .child(item.label().clone())
                .when(selected == Some(index), |option| option.child("✓"))
        });
        frame(theme).items_start().pt(px(48.)).child(
            Select::new("fruit", &self.state)
                .accessibility_label("Fruit")
                .w(px(200.))
                .rounded(px(6.))
                .border_1()
                .border_color(theme.border)
                .bg(theme.background)
                .focus_visible(|style| style.border_color(theme.ring))
                .trigger(
                    div()
                        .h(px(36.))
                        .px(px(12.))
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .when(!has_value, |text| text.text_color(theme.muted_foreground))
                                .child(current),
                        )
                        .child(div().text_color(theme.muted_foreground).child("⌄")),
                )
                .content(popup(theme).w(px(200.)).children(options)),
        )
    }
}

fn select(theme: Theme, cx: &mut App) -> AnyView {
    let state = cx.new(|cx| {
        SelectState::new(
            [
                SelectItem::new("apple", "Apple"),
                SelectItem::new("banana", "Banana"),
                SelectItem::new("blueberry", "Blueberry"),
                SelectItem::new("grapes", "Grapes").disabled(true),
                SelectItem::new("pineapple", "Pineapple"),
            ],
            cx,
        )
    });
    cx.new(|cx| {
        let observe = cx.observe(&state, |_, _, cx| cx.notify());
        SelectDemo {
            theme,
            state,
            _observe: observe,
        }
    })
    .into()
}

// Combobox

struct ComboboxDemo {
    theme: Theme,
    state: Entity<ComboboxState>,
    _observe: Subscription,
}

impl Render for ComboboxDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        let state = self.state.read(cx);
        let highlighted = state.highlighted_index(cx);
        let visible = state.visible_indices(cx).to_vec();
        let options = visible.iter().map(|&index| {
            let item = &state.items(cx)[index];
            ComboboxOption::new(&self.state, index)
                .h(px(32.))
                .px(px(8.))
                .flex()
                .items_center()
                .justify_between()
                .rounded(px(6.))
                .when(highlighted == Some(index), |option| option.bg(theme.accent))
                .when(item.is_disabled(), |option| option.opacity(0.5))
                .child(item.label().clone())
                .when(state.is_selected(index), |option| option.child("✓"))
        });
        let list = ComboboxList::new("framework-options", &self.state)
            .accessibility_label("Frameworks")
            .max_h(px(180.))
            .children(options);
        let selected = state
            .selected_items(cx)
            .next()
            .map(|item| format!("Selected: {}", item.label()))
            .unwrap_or_else(|| "No framework selected".into());
        frame(theme).flex_col().justify_start().pt(px(32.)).child(
            div()
                .w(px(240.))
                .flex()
                .flex_col()
                .gap(px(8.))
                .child(
                    Combobox::new("framework", &self.state)
                        .input(|input| {
                            input
                                .accessibility_label("Framework")
                                .placeholder("Search framework…")
                                .h(px(36.))
                                .w_full()
                                .px(px(12.))
                                .rounded(px(6.))
                                .border_1()
                                .border_color(theme.border)
                                .bg(theme.background)
                                .styles(|styles| {
                                    styles.focused(|style| style.border_color(theme.ring))
                                })
                        })
                        .content(popup(theme).w(px(240.)).child(list).when(
                            visible.is_empty(),
                            |surface| {
                                surface.child(
                                    div()
                                        .p(px(8.))
                                        .text_color(theme.muted_foreground)
                                        .child("No framework found."),
                                )
                            },
                        )),
                )
                .child(muted(theme, selected)),
        )
    }
}

fn combobox(theme: Theme, window: &mut Window, cx: &mut App) -> AnyView {
    let state = cx.new(|cx| {
        ComboboxState::new(
            [
                SearchItem::new("next", "Next.js"),
                SearchItem::new("sveltekit", "SvelteKit"),
                SearchItem::new("nuxt", "Nuxt.js"),
                SearchItem::new("remix", "Remix"),
                SearchItem::new("astro", "Astro"),
            ],
            window,
            cx,
        )
    });
    state
        .read(cx)
        .input()
        .clone()
        .update(cx, |input, _| input.set_editor_style(editor_style(theme)));
    cx.new(|cx| {
        let observe = cx.observe(&state, |_, _, cx| cx.notify());
        ComboboxDemo {
            theme,
            state,
            _observe: observe,
        }
    })
    .into()
}

// Input OTP

struct InputOtpDemo {
    theme: Theme,
    state: Entity<InputOtpState>,
    status: SharedString,
    _subscriptions: Vec<Subscription>,
}

impl Render for InputOtpDemo {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        frame(theme)
            .flex_col()
            .child(
                InputOtp::new("one-time-password", &self.state)
                    .accessibility_label("One-time password")
                    .accessibility_description("Six digits.")
                    .flex()
                    .gap(px(8.))
                    .slot(move |slot, _, _| {
                        let current = slot.active || slot.selected;
                        div()
                            .size(px(40.))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded(px(6.))
                            .border_1()
                            .border_color(if current { theme.ring } else { theme.border })
                            .when(current, |slot| slot.bg(theme.accent))
                            .when(slot.index == 2, |slot| slot.mr(px(12.)))
                            .text_size(px(16.))
                            .child(match slot.character {
                                Some(character) => character.to_string(),
                                None if slot.active => "|".to_string(),
                                None => String::new(),
                            })
                    }),
            )
            .child(muted(theme, self.status.clone()))
    }
}

fn input_otp(theme: Theme, window: &mut Window, cx: &mut App) -> AnyView {
    let state = cx.new(|cx| InputOtpState::new(6, window, cx));
    cx.new(|cx| {
        let subscriptions = vec![
            cx.observe(&state, |_, _, cx| cx.notify()),
            cx.subscribe(
                &state,
                |this: &mut InputOtpDemo, _, event: &InputOtpEvent, cx| {
                    this.status = if event.complete {
                        format!("Entered {}.", event.value).into()
                    } else {
                        "Enter your one-time password.".into()
                    };
                    cx.notify();
                },
            ),
        ];
        InputOtpDemo {
            theme,
            state,
            status: "Enter your one-time password.".into(),
            _subscriptions: subscriptions,
        }
    })
    .into()
}

// Calendar and DatePicker

fn calendar_state(selected: Option<NaiveDate>, cx: &mut App) -> Entity<CalendarState> {
    cx.new(|cx| {
        CalendarState::new(today(), cx)
            .bounds(
                NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(2027, 12, 31).unwrap(),
            )
            .default_selection(CalendarSelection::Single(selected))
    })
}

fn month_button(
    theme: Theme,
    id: &'static str,
    label: &'static str,
    glyph: &'static str,
) -> Button {
    outline(theme, Button::new(id))
        .accessibility_label(label)
        .size(px(28.))
        .px(px(0.))
        .child(glyph)
}

/// A month grid with previous/next controls, shared by both date demos.
fn calendar_card(theme: Theme, state: &Entity<CalendarState>, cx: &App) -> Div {
    let model = state.read(cx);
    let month = model.month();
    let month_key = month.format("%Y-%m").to_string();
    let title = month.format("%B %Y").to_string();
    let previous = state.clone();
    let next = state.clone();
    let grid = Calendar::new("calendar", state)
        .accessibility_label(title.clone())
        .flex()
        .flex_col()
        .gap(px(2.))
        .child(
            div()
                .id("weekdays")
                .role(Role::Row)
                .aria_row_index(0)
                .flex()
                .children(
                    ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"]
                        .into_iter()
                        .enumerate()
                        .map(|(column, day)| {
                            div()
                                .id(("weekday", column))
                                .role(Role::ColumnHeader)
                                .aria_column_index(column)
                                .aria_label(day)
                                .w(px(32.))
                                .text_center()
                                .text_size(px(12.))
                                .text_color(theme.muted_foreground)
                                .child(day)
                        }),
                ),
        )
        .children(model.days().chunks(7).enumerate().map(|(row, week)| {
            div()
                .id(("week", row))
                .role(Role::Row)
                .aria_row_index(row + 1)
                .flex()
                .children(week.iter().map(|&date| {
                    let selected = model.is_selected(date);
                    let outside = date.format("%Y-%m").to_string() != month_key;
                    CalendarDay::new(state, date)
                        .accessibility_label(date.format("%A, %B %-d, %Y").to_string())
                        .size(px(32.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded(px(6.))
                        .border_1()
                        .border_color(gpui::transparent_black())
                        .when(outside, |day| day.text_color(theme.muted_foreground))
                        .when(date == today() && !selected, |day| day.bg(theme.accent))
                        .hover(|style| style.bg(theme.accent))
                        .focus_visible(|style| style.border_color(theme.ring))
                        .styles(|styles| styles.disabled(|style| style.opacity(0.4)))
                        .when(selected, |day| {
                            day.bg(theme.primary).text_color(theme.primary_foreground)
                        })
                        .child(date.format("%-d").to_string())
                }))
        }));
    popup(theme)
        .p(px(12.))
        .flex()
        .flex_col()
        .gap(px(8.))
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(
                    month_button(theme, "previous-month", "Previous month", "‹")
                        .disabled(!model.can_previous_month())
                        .on_click(move |_, window, cx| {
                            previous.update(cx, |state, cx| state.previous_month(window, cx));
                        }),
                )
                .child(div().font_weight(FontWeight::MEDIUM).child(title))
                .child(
                    month_button(theme, "next-month", "Next month", "›")
                        .disabled(!model.can_next_month())
                        .on_click(move |_, window, cx| {
                            next.update(cx, |state, cx| state.next_month(window, cx));
                        }),
                ),
        )
        .child(grid)
}

struct CalendarDemo {
    theme: Theme,
    state: Entity<CalendarState>,
    _observe: Subscription,
}

impl Render for CalendarDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        let selected = self
            .state
            .read(cx)
            .selection()
            .first_date()
            .map(|date| date.format("Selected %A, %B %-d").to_string())
            .unwrap_or_else(|| "No date selected".into());
        frame(theme)
            .flex_col()
            .gap(px(8.))
            .child(calendar_card(theme, &self.state, cx))
            .child(muted(theme, selected).text_size(px(13.)))
    }
}

fn calendar(theme: Theme, cx: &mut App) -> AnyView {
    let state = calendar_state(Some(today()), cx);
    cx.new(|cx| {
        let observe = cx.observe(&state, |_, _, cx| cx.notify());
        CalendarDemo {
            theme,
            state,
            _observe: observe,
        }
    })
    .into()
}

struct DatePickerDemo {
    theme: Theme,
    calendar: Entity<CalendarState>,
    picker: Entity<DatePickerState>,
    _observe: Subscription,
}

impl Render for DatePickerDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        let date = self.picker.read(cx).selection(cx).first_date();
        let text = date
            .map(|date| date.format("%B %-d, %Y").to_string())
            .unwrap_or_else(|| "Pick a date".into());
        let trigger_label = format!("Date, {text}");
        frame(theme).items_start().pt(px(12.)).child(
            DatePicker::new("date", &self.picker)
                .accessibility_label("Choose a date")
                .offset(px(6.))
                .trigger(move |button| {
                    outline(theme, button)
                        .w(px(240.))
                        .px(px(12.))
                        .justify_start()
                        .gap(px(8.))
                        .accessibility_label(trigger_label)
                        .when(date.is_none(), |button| {
                            button.text_color(theme.muted_foreground)
                        })
                        .child("▦")
                        .child(text)
                })
                .content(calendar_card(theme, &self.calendar, cx)),
        )
    }
}

fn date_picker(theme: Theme, window: &mut Window, cx: &mut App) -> AnyView {
    let calendar = calendar_state(None, cx);
    let picker = cx.new(|cx| DatePickerState::new(&calendar, window, cx));
    cx.new(|cx| {
        let observe = cx.observe(&picker, |_, _, cx| cx.notify());
        DatePickerDemo {
            theme,
            calendar,
            picker,
            _observe: observe,
        }
    })
    .into()
}

// Table

const INVOICES: [(&str, &str, &str, u32); 5] = [
    ("INV001", "Paid", "Credit Card", 25000),
    ("INV002", "Pending", "PayPal", 15000),
    ("INV003", "Unpaid", "Bank Transfer", 35000),
    ("INV004", "Paid", "Credit Card", 45000),
    ("INV005", "Paid", "PayPal", 55000),
];

fn dollars(cents: u32) -> String {
    format!("${}.{:02}", cents / 100, cents % 100)
}

struct TableDemo {
    theme: Theme,
    sort: Option<SortDirection>,
}

impl Render for TableDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        let mut rows = INVOICES.to_vec();
        match self.sort {
            Some(SortDirection::Ascending) => rows.sort_by_key(|row| row.3),
            Some(SortDirection::Descending) => rows.sort_by_key(|row| std::cmp::Reverse(row.3)),
            _ => {}
        }
        let cell = |id: ElementId| {
            TableCell::new(id)
                .h(px(40.))
                .px(px(8.))
                .flex()
                .items_center()
        };
        let arrow = match self.sort {
            Some(SortDirection::Ascending) => " ↑",
            Some(SortDirection::Descending) => " ↓",
            _ => " ↕",
        };
        let header = TableRow::new("header")
            .border_b_1()
            .border_color(theme.border)
            .text_color(theme.muted_foreground)
            .font_weight(FontWeight::MEDIUM)
            .cell(cell("h-invoice".into()).child("Invoice"))
            .cell(cell("h-status".into()).child("Status"))
            .cell(cell("h-method".into()).child("Method"))
            .cell(
                cell("h-amount".into())
                    .justify_end()
                    .sort_direction(self.sort)
                    .child(
                        Button::new("sort-amount")
                            .accessibility_label("Sort by amount")
                            .px(px(6.))
                            .h(px(28.))
                            .flex()
                            .items_center()
                            .rounded(px(6.))
                            .hover(|style| style.bg(theme.accent))
                            .focus_visible(|style| style.bg(theme.accent))
                            .child(format!("Amount{arrow}"))
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.sort = match this.sort {
                                    Some(SortDirection::Ascending) => {
                                        Some(SortDirection::Descending)
                                    }
                                    _ => Some(SortDirection::Ascending),
                                };
                                cx.notify();
                            })),
                    ),
            );
        let body = rows.iter().map(|(invoice, status, method, amount)| {
            TableRow::new(SharedString::from(*invoice))
                .border_b_1()
                .border_color(theme.border)
                .cell(
                    cell(ElementId::Name(format!("{invoice}-id").into()))
                        .row_header(true)
                        .font_weight(FontWeight::MEDIUM)
                        .child(*invoice),
                )
                .cell(cell(ElementId::Name(format!("{invoice}-status").into())).child(*status))
                .cell(cell(ElementId::Name(format!("{invoice}-method").into())).child(*method))
                .cell(
                    cell(ElementId::Name(format!("{invoice}-amount").into()))
                        .justify_end()
                        .child(dollars(*amount)),
                )
        });
        let total: u32 = INVOICES.iter().map(|row| row.3).sum();
        frame(theme).child(
            Table::new(
                "invoices",
                [
                    TableColumn::fixed(px(100.)),
                    TableColumn::fixed(px(100.)),
                    TableColumn::fixed(px(130.)),
                    TableColumn::fixed(px(110.)),
                ],
            )
            .accessibility_label("Invoices")
            .w(px(440.))
            .caption(
                TableCaption::new("invoices-caption")
                    .pt(px(12.))
                    .text_color(theme.muted_foreground)
                    .child("A list of your recent invoices."),
            )
            .header(header)
            .rows(body)
            .footer(
                TableRow::new("footer")
                    .bg(theme.muted)
                    .font_weight(FontWeight::MEDIUM)
                    .cell(cell("f-label".into()).child("Total"))
                    .cell(cell("f-status".into()))
                    .cell(cell("f-method".into()))
                    .cell(cell("f-amount".into()).justify_end().child(dollars(total))),
            ),
        )
    }
}

fn table(theme: Theme, cx: &mut App) -> AnyView {
    cx.new(|_| TableDemo { theme, sort: None }).into()
}

// ScrollArea

struct ScrollAreaDemo {
    theme: Theme,
    scroll: ScrollHandle,
}

impl Render for ScrollAreaDemo {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        let tags = (1..=50).rev().map(|index| {
            div()
                .py(px(8.))
                .border_b_1()
                .border_color(theme.border)
                .child(format!("v1.2.0-beta.{index}"))
        });
        frame(theme).child(
            ScrollArea::new("tags", &self.scroll)
                .accessibility_label("Tags")
                .scrollbar_mode(ScrollbarMode::Always)
                .scrollbar_styles(|styles| {
                    styles.thumb(|thumb| thumb.bg(theme.border).radius(px(4.)))
                })
                .w(px(200.))
                .h(px(280.))
                .rounded(px(8.))
                .border_1()
                .border_color(theme.border)
                .content_styles(|style| style.p(px(16.)))
                .child(heading("Tags").pb(px(8.)))
                .children(tags),
        )
    }
}

fn scroll_area(theme: Theme, cx: &mut App) -> AnyView {
    cx.new(|_| ScrollAreaDemo {
        theme,
        scroll: ScrollHandle::new(),
    })
    .into()
}

// Resizable

struct ResizableDemo {
    theme: Theme,
    outer: Entity<ResizableState>,
    inner: Entity<ResizableState>,
    _observe: [Subscription; 2],
}

fn pane(name: &'static str, share: f32) -> Div {
    div()
        .size_full()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap(px(4.))
        .child(heading(name))
        .child(format!("{:.0}%", share * 100.))
}

fn grip(
    theme: Theme,
    axis: Axis,
    handle: gpui_design::ResizableHandle,
) -> gpui_design::ResizableHandle {
    let line = div().bg(theme.border);
    let horizontal = axis == Axis::Horizontal;
    handle
        .flex()
        .items_center()
        .justify_center()
        .when(horizontal, |handle| handle.w(px(9.)).h_full())
        .when(!horizontal, |handle| handle.h(px(9.)).w_full())
        .focus_visible(|style| style.bg(theme.accent))
        .child(if horizontal {
            line.w(px(1.)).h_full()
        } else {
            line.h(px(1.)).w_full()
        })
}

impl Render for ResizableDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        let outer = self.outer.read(cx).sizes().to_vec();
        let inner = self.inner.read(cx).sizes().to_vec();
        let nested = Resizable::new("content-split", &self.inner)
            .size_full()
            .panel(pane("Header", inner[0]))
            .handle(move |_, handle| {
                grip(
                    theme,
                    Axis::Vertical,
                    handle.accessibility_label("Resize header"),
                )
            })
            .panel(pane("Content", inner[1]));
        frame(theme).child(
            Resizable::new("layout-split", &self.outer)
                .w(px(400.))
                .h(px(240.))
                .rounded(px(8.))
                .border_1()
                .border_color(theme.border)
                .overflow_hidden()
                .panel(pane("Sidebar", outer[0]))
                .handle(move |_, handle| {
                    grip(
                        theme,
                        Axis::Horizontal,
                        handle.accessibility_label("Resize sidebar"),
                    )
                })
                .panel(nested),
        )
    }
}

fn resizable(theme: Theme, cx: &mut App) -> AnyView {
    let outer = cx.new(|cx| {
        ResizableState::new(
            Axis::Horizontal,
            [
                ResizablePanel::new(0.4).limits(0.2..=0.7),
                ResizablePanel::new(0.6).limits(0.3..=0.8),
            ],
            cx,
        )
    });
    let inner = cx.new(|cx| {
        ResizableState::new(
            Axis::Vertical,
            [
                ResizablePanel::new(0.3).limits(0.2..=0.6),
                ResizablePanel::new(0.7).limits(0.4..=0.8),
            ],
            cx,
        )
    });
    cx.new(|cx| {
        let observe = [
            cx.observe(&outer, |_, _, cx| cx.notify()),
            cx.observe(&inner, |_, _, cx| cx.notify()),
        ];
        ResizableDemo {
            theme,
            outer,
            inner,
            _observe: observe,
        }
    })
    .into()
}

// Carousel

const SLIDE_HUES: [f32; 5] = [0.61, 0.47, 0.08, 0.83, 0.36];

struct CarouselDemo {
    theme: Theme,
    state: Entity<CarouselState>,
    _observe: Subscription,
}

impl Render for CarouselDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        let model = self.state.read(cx);
        let selected = model.selected_index().unwrap_or(0);
        let count = model.labels().len();
        let (can_previous, can_next) = (model.can_previous(), model.can_next());
        let control = |control: CarouselControl, enabled: bool, glyph: &'static str| {
            control
                .size(px(32.))
                .flex()
                .items_center()
                .justify_center()
                .rounded_full()
                .border_1()
                .border_color(theme.border)
                .bg(theme.background)
                .when(!enabled, |control| control.opacity(0.5))
                .child(glyph)
        };
        let mut carousel = Carousel::new("featured", &self.state, "Featured slides")
            .w(px(220.))
            .flex();
        for (index, hue) in SLIDE_HUES.into_iter().enumerate() {
            carousel = carousel.child(
                CarouselSlide::new(&self.state, index)
                    .flex_1()
                    .h(px(200.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded(px(8.))
                    .bg(hsla(hue, 0.65, 0.5, 1.))
                    .text_color(gpui::white())
                    .text_size(px(40.))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(format!("{}", index + 1)),
            );
        }
        frame(theme)
            .flex_col()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(16.))
                    .child(control(
                        CarouselControl::previous("previous-slide", &self.state),
                        can_previous,
                        "←",
                    ))
                    .child(carousel)
                    .child(control(
                        CarouselControl::next("next-slide", &self.state),
                        can_next,
                        "→",
                    )),
            )
            .child(muted(theme, format!("Slide {} of {count}", selected + 1)))
    }
}

fn carousel(theme: Theme, cx: &mut App) -> AnyView {
    let state = cx
        .new(|cx| CarouselState::new(["Slide 1", "Slide 2", "Slide 3", "Slide 4", "Slide 5"], cx));
    cx.new(|cx| {
        let observe = cx.observe(&state, |_, _, cx| cx.notify());
        CarouselDemo {
            theme,
            state,
            _observe: observe,
        }
    })
    .into()
}

// Chart

struct ChartDemo {
    theme: Theme,
    state: Entity<ChartState>,
    _observe: Subscription,
}

impl ChartDemo {
    fn colors(theme: Theme) -> [Hsla; 2] {
        if theme.background.l > 0.5 {
            [gpui::rgb(0x2563eb).into(), gpui::rgb(0x60a5fa).into()]
        } else {
            [gpui::rgb(0x3b82f6).into(), gpui::rgb(0x93c5fd).into()]
        }
    }
}

impl Render for ChartDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        let colors = Self::colors(theme);
        let model = self.state.read(cx);
        let categories = model.categories().to_vec();
        let inspected = model.inspected_index();
        let visible: Vec<usize> = (0..model.series().len())
            .filter(|&series| model.is_visible(series))
            .collect();
        // Normalized bar heights per category, in visible series order.
        let bars: Vec<Vec<(f32, Hsla)>> = (0..categories.len())
            .map(|category| {
                visible
                    .iter()
                    .map(|&series| {
                        let value = model.series()[series].values()[category];
                        (model.normalized_value(value), colors[series])
                    })
                    .collect()
            })
            .collect();
        let description = model.inspected_description();
        let accent = theme.accent;
        let plot = canvas(
            |_, _, _| (),
            move |bounds: Bounds<gpui::Pixels>, _, window, _| {
                let band = bounds.size.width / bars.len().max(1) as f32;
                for (category, group) in bars.iter().enumerate() {
                    let left = bounds.left() + band * category as f32;
                    if inspected == Some(category) {
                        window.paint_quad(
                            fill(
                                Bounds::new(
                                    point(left, bounds.top()),
                                    size(band, bounds.size.height),
                                ),
                                accent,
                            )
                            .corner_radii(px(4.)),
                        );
                    }
                    let inner = band * 0.7;
                    let gap = px(4.);
                    let width =
                        (inner - gap * (group.len().max(1) - 1) as f32) / group.len().max(1) as f32;
                    for (slot, (height, color)) in group.iter().enumerate() {
                        let height = (bounds.size.height - px(8.)) * *height;
                        let x = left + (band - inner) / 2. + (width + gap) * slot as f32;
                        window.paint_quad(
                            fill(
                                Bounds::new(
                                    point(x, bounds.bottom() - height),
                                    size(width, height),
                                ),
                                *color,
                            )
                            .corner_radii(px(3.)),
                        );
                    }
                }
            },
        )
        .size_full();
        let legend = model.series().iter().enumerate().map(|(index, series)| {
            let pressed = model.is_visible(index);
            let state = self.state.clone();
            Toggle::new(("legend", index))
                .accessibility_label(format!("Show {}", series.label()))
                .pressed(pressed)
                .h(px(28.))
                .px(px(8.))
                .flex()
                .items_center()
                .gap(px(6.))
                .rounded(px(6.))
                .hover(|style| style.bg(theme.accent))
                .focus_visible(|style| style.bg(theme.accent))
                .when(!pressed, |toggle| toggle.opacity(0.5))
                .child(div().size(px(10.)).rounded(px(2.)).bg(colors[index]))
                .child(series.label().clone())
                .on_change(move |pressed, _, window, cx| {
                    state.update(cx, |state, cx| {
                        state.set_visible(index, pressed, window, cx);
                    });
                })
        });
        frame(theme).flex_col().gap(px(8.)).child(
            div()
                .w(px(380.))
                .flex()
                .flex_col()
                .gap(px(6.))
                .child(
                    Chart::new("visitors", &self.state, "Visitors by month")
                        .w_full()
                        .h(px(170.))
                        .child(plot),
                )
                .child(
                    div()
                        .flex()
                        .children(categories.into_iter().map(|category| {
                            div()
                                .flex_1()
                                .text_center()
                                .text_size(px(12.))
                                .text_color(theme.muted_foreground)
                                .child(category)
                        })),
                )
                .child(div().flex().justify_center().gap(px(8.)).children(legend))
                .child(muted(theme, description).text_size(px(13.)).text_center()),
        )
    }
}

fn chart(theme: Theme, cx: &mut App) -> AnyView {
    let state = cx.new(|cx| {
        ChartState::new(
            ["Jan", "Feb", "Mar", "Apr", "May", "Jun"],
            [
                ChartSeries::new("Desktop", [186., 305., 237., 73., 209., 214.]),
                ChartSeries::new("Mobile", [80., 200., 120., 190., 130., 140.]),
            ],
            cx,
        )
    });
    cx.new(|cx| {
        let observe = cx.observe(&state, |_, _, cx| cx.notify());
        ChartDemo {
            theme,
            state,
            _observe: observe,
        }
    })
    .into()
}

// Direction

struct DirectionDemo {
    theme: Theme,
    direction: Direction,
    group: Entity<ToggleGroupState>,
    _observe: Subscription,
}

impl Render for DirectionDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        let direction = self.direction;
        let group = self.group.read(cx);
        let options = (0..group.items().len()).map(|index| {
            ToggleOption::new(&self.group, index)
                .h(px(32.))
                .px(px(12.))
                .flex()
                .items_center()
                .rounded(px(6.))
                .when(group.is_pressed(index), |option| option.bg(theme.accent))
                .hover(|style| style.bg(theme.accent))
                .focus_visible(|style| style.border_1().border_color(theme.ring))
                .child(group.items()[index].label().clone())
        });
        let rtl = direction.is_rtl();
        frame(theme)
            .flex_col()
            .child(
                DirectionProvider::new(direction)
                    .w(px(320.))
                    .flex()
                    .flex_col()
                    .gap(px(12.))
                    .child(
                        direction.row(
                            ToggleGroup::new("steps", &self.group)
                                .accessibility_label("Step")
                                .flex()
                                .gap(px(4.))
                                .p(px(4.))
                                .rounded(px(8.))
                                .border_1()
                                .border_color(theme.border),
                        )
                        .children(options),
                    )
                    .child(muted(
                        theme,
                        "Focus a step, then use Left and Right. Arrows follow the reading direction.",
                    )),
            )
            .child(
                outline(theme, Button::new("toggle-direction"))
                    .accessibility_label(if rtl { "Switch to left-to-right" } else { "Switch to right-to-left" })
                    .child(if rtl { "Direction: RTL" } else { "Direction: LTR" })
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.direction = if this.direction.is_rtl() {
                            Direction::Ltr
                        } else {
                            Direction::Rtl
                        };
                        cx.notify();
                    })),
            )
    }
}

fn direction(theme: Theme, cx: &mut App) -> AnyView {
    let group = cx.new(|cx| {
        ToggleGroupState::new(
            [
                ToggleItem::new("account", "Account"),
                ToggleItem::new("shipping", "Shipping"),
                ToggleItem::new("payment", "Payment"),
            ],
            cx,
        )
        .single_selection()
    });
    cx.new(|cx| {
        let observe = cx.observe(&group, |_, _, cx| cx.notify());
        DirectionDemo {
            theme,
            direction: Direction::Rtl,
            group,
            _observe: observe,
        }
    })
    .into()
}

// MessageScroller

struct MessageScrollerDemo {
    theme: Theme,
    state: Entity<MessageScrollerState>,
    messages: Vec<(usize, SharedString, bool)>,
    _observe: Subscription,
}

const REPLIES: [&str; 4] = [
    "Sounds good. I will review the draft this afternoon.",
    "Can you share the latest numbers?",
    "Thanks! That fixed it.",
    "Let's sync tomorrow at 10.",
];

impl Render for MessageScrollerDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        let model = self.state.read(cx);
        let unread = model.unread_count();
        let following = model.is_following();
        let mut area = MessageScroller::new("messages", &self.state, cx)
            .accessibility_label("Conversation")
            .w(px(340.))
            .h(px(220.))
            .rounded(px(8.))
            .border_1()
            .border_color(theme.border)
            .content_styles(|style| style.flex().flex_col().gap(px(8.)).p(px(12.)));
        for (id, body, outgoing) in &self.messages {
            let outgoing = *outgoing;
            area = area.item(
                *id,
                div().flex().when(outgoing, |row| row.justify_end()).child(
                    div()
                        .max_w(px(240.))
                        .px(px(12.))
                        .py(px(8.))
                        .rounded(px(8.))
                        .when(outgoing, |bubble| {
                            bubble
                                .bg(theme.primary)
                                .text_color(theme.primary_foreground)
                        })
                        .when(!outgoing, |bubble| bubble.bg(theme.muted))
                        .child(body.clone()),
                ),
            );
        }
        frame(theme).flex_col().gap(px(12.)).child(area).child(
            div()
                .flex()
                .gap(px(8.))
                .child(
                    outline(theme, Button::new("receive"))
                        .accessibility_label("Receive message")
                        .child("Receive message")
                        .on_click(cx.listener(|this, _, _, cx| {
                            let id = this.messages.len();
                            let body = REPLIES[id % REPLIES.len()];
                            this.messages.push((id, body.into(), false));
                            cx.notify();
                        })),
                )
                .child(
                    primary(theme, Button::new("jump"))
                        .accessibility_label("Jump to latest")
                        .disabled(following)
                        .child(if unread > 0 {
                            format!("{unread} new · Jump to latest")
                        } else {
                            "Jump to latest".to_string()
                        })
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.state
                                .update(cx, |state, cx| state.jump_to_latest(window, cx));
                        })),
                ),
        )
    }
}

fn message_scroller(theme: Theme, cx: &mut App) -> AnyView {
    let state = cx.new(MessageScrollerState::new);
    let messages = (0..10)
        .map(|id| {
            let outgoing = id % 3 == 1;
            let body = if outgoing {
                "Here is the update you asked for."
            } else {
                REPLIES[id % REPLIES.len()]
            };
            (id, SharedString::from(body), outgoing)
        })
        .collect();
    cx.new(|cx| {
        let observe = cx.observe(&state, |_, _, cx| cx.notify());
        MessageScrollerDemo {
            theme,
            state,
            messages,
            _observe: observe,
        }
    })
    .into()
}

// Questionnaire

const QUESTIONS: [(&str, &str); 2] = [
    ("Project name", "Acme dashboard"),
    ("Your role", "Designer, engineer…"),
];

struct QuestionnaireDemo {
    theme: Theme,
    answers: [Entity<InputState>; 2],
    state: QuestionnaireState<SharedString>,
    next_focus: FocusHandle,
    back_focus: FocusHandle,
    _observe: [Subscription; 2],
}

impl QuestionnaireDemo {
    fn focus_stage(&self, window: &mut Window, cx: &mut Context<Self>) {
        match self.state.stage() {
            QuestionnaireStage::Question(index) => {
                self.answers[index].focus_handle(cx).focus(window, cx)
            }
            QuestionnaireStage::Review => self.next_focus.focus(window, cx),
            QuestionnaireStage::Complete => self.back_focus.focus(window, cx),
        }
    }

    fn advance(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        match self.state.stage() {
            QuestionnaireStage::Question(index) => {
                let value = self.answers[index].read(cx).value();
                let validation = if value.trim().is_empty() {
                    Err("Enter an answer to continue.".into())
                } else {
                    Ok(())
                };
                self.state.submit_answer(value, validation);
            }
            QuestionnaireStage::Review => {
                self.state.complete();
            }
            QuestionnaireStage::Complete => return,
        }
        self.focus_stage(window, cx);
        cx.notify();
    }
}

impl Render for QuestionnaireDemo {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        let stage = self.state.stage();
        let title = match stage {
            QuestionnaireStage::Question(index) => format!("Question {} of 2", index + 1),
            QuestionnaireStage::Review => "Review your answers".to_string(),
            QuestionnaireStage::Complete => "All done".to_string(),
        };
        let mut card = Questionnaire::new("setup", title.clone())
            .w(px(320.))
            .flex()
            .flex_col()
            .gap(px(10.))
            .p(px(16.))
            .rounded(px(8.))
            .border_1()
            .border_color(theme.border)
            .child(div().font_weight(FontWeight::SEMIBOLD).child(title));
        match stage {
            QuestionnaireStage::Question(index) => {
                let error = self.state.error().cloned();
                let input = &self.answers[index];
                card = card
                    .child(
                        Label::new(
                            ("question", index),
                            &input.focus_handle(cx),
                            QUESTIONS[index].0,
                        )
                        .font_weight(FontWeight::MEDIUM),
                    )
                    .child(input_frame(
                        theme,
                        ("answer", index),
                        input,
                        error.is_some(),
                        window,
                        cx,
                    ))
                    .when_some(error, |card, error| {
                        card.child(div().text_color(theme.destructive).child(error))
                    });
            }
            QuestionnaireStage::Review | QuestionnaireStage::Complete => {
                for (index, (question, _)) in QUESTIONS.iter().enumerate() {
                    let answer = self.state.answer(index).cloned().unwrap_or_default();
                    card = card.child(
                        div()
                            .flex()
                            .justify_between()
                            .child(muted(theme, *question))
                            .child(answer),
                    );
                }
                if stage == QuestionnaireStage::Complete {
                    card = card.child(muted(theme, "Setup complete. Back reopens the review."));
                }
            }
        }
        let next_label = match stage {
            QuestionnaireStage::Question(_) => "Continue",
            QuestionnaireStage::Review => "Submit",
            QuestionnaireStage::Complete => "Submitted",
        };
        frame(theme).child(
            card.child(
                div()
                    .flex()
                    .justify_end()
                    .gap(px(8.))
                    .child(
                        outline(theme, Button::new("back"))
                            .accessibility_label("Back")
                            .track_focus(&self.back_focus)
                            .disabled(!self.state.can_go_back())
                            .child("Back")
                            .on_click(cx.listener(|this, _, window, cx| {
                                if this.state.back() {
                                    this.focus_stage(window, cx);
                                    cx.notify();
                                }
                            })),
                    )
                    .child(
                        primary(theme, Button::new("next"))
                            .accessibility_label(next_label)
                            .track_focus(&self.next_focus)
                            .disabled(stage == QuestionnaireStage::Complete)
                            .child(next_label)
                            .on_click(cx.listener(|this, _, window, cx| this.advance(window, cx))),
                    ),
            ),
        )
    }
}

fn questionnaire(theme: Theme, window: &mut Window, cx: &mut App) -> AnyView {
    let answers =
        QUESTIONS.map(|(label, placeholder)| text_input(theme, label, placeholder, window, cx));
    cx.new(|cx| {
        let observe = [
            cx.observe(&answers[0], |_, _, cx| cx.notify()),
            cx.observe(&answers[1], |_, _, cx| cx.notify()),
        ];
        QuestionnaireDemo {
            theme,
            answers,
            state: QuestionnaireState::new(2),
            next_focus: cx.focus_handle(),
            back_focus: cx.focus_handle(),
            _observe: observe,
        }
    })
    .into()
}
