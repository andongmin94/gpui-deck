//! Label, help, and validation presentation associated with one retained input.

use std::rc::Rc;

use gpui::{
    AnyElement, App, Div, ElementId, Entity, FocusHandle, InteractiveElement as _, IntoElement,
    MouseButton, ParentElement, Refineable as _, RenderOnce, Role, SharedString,
    StatefulInteractiveElement as _, StyleRefinement, Styled, Window, div,
    prelude::FluentBuilder as _,
};

use crate::{
    StateStyle, StyledExt as _, TestSupportExt as _,
    a11y_disabled::A11yDisabledExt as _,
    input::{InputAccessibility, InputBaseState, InputModeKind},
};

/// An unstyled field associated with an existing Input or Textarea state.
///
/// The field renders its label, application-provided control children, help,
/// and optional error in that order. Applications supply layout, styles, and
/// validation; this component neither stores text nor validates it. Clicking
/// the label focuses the live enabled input without introducing another Tab
/// stop. The label's element ID is `(field_id, "label")`.
///
/// Label, error/help description, required, and invalid metadata are projected
/// onto the input's existing native node before its child renderer runs. Its
/// role, placeholder, external identifier, value exposure, and popup metadata
/// are preserved. An error is announced before help text. When no semantics
/// were configured, the input's mode supplies its natural text-input role.
///
/// Keep the associated input among this field's children. A composed input
/// renderer must likewise preserve metadata it does not own. Host application
/// content in [`crate::Root`] for normal form traversal.
#[derive(IntoElement)]
pub struct Field<M: InputModeKind> {
    id: ElementId,
    base: crate::ObservedElement<gpui::Stateful<Div>>,
    input: Entity<InputBaseState<M>>,
    label: SharedString,
    description: Option<SharedString>,
    error: Option<SharedString>,
    required: bool,
    styles: FieldStyles,
    children: Vec<AnyElement>,
}

impl<M: InputModeKind> Field<M> {
    pub fn new(
        id: impl Into<ElementId>,
        input: &Entity<InputBaseState<M>>,
        label: impl Into<SharedString>,
    ) -> Self {
        let id = id.into();
        Self {
            base: div().id(id.clone()).test_support(),
            id,
            input: input.clone(),
            label: label.into(),
            description: None,
            error: None,
            required: false,
            styles: FieldStyles::default(),
            children: Vec::new(),
        }
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Supplies an application validation error; `None` clears invalid state.
    pub fn error(mut self, error: Option<SharedString>) -> Self {
        self.error = error;
        self
    }

    /// Reports the requirement to accessibility clients; does not reject input.
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn styles(mut self, build: impl FnOnce(FieldStyles) -> FieldStyles) -> Self {
        self.styles = build(self.styles);
        self
    }
}

/// Application-owned presentation of the field's text parts.
#[derive(Default)]
pub struct FieldStyles {
    label: StyleRefinement,
    description: StyleRefinement,
    error: StyleRefinement,
}

impl FieldStyles {
    pub fn label(mut self, build: impl FnOnce(StateStyle) -> StateStyle) -> Self {
        self.label
            .refine(&build(StateStyle::default()).into_refinement());
        self
    }

    pub fn description(mut self, build: impl FnOnce(StateStyle) -> StateStyle) -> Self {
        self.description
            .refine(&build(StateStyle::default()).into_refinement());
        self
    }

    pub fn error(mut self, build: impl FnOnce(StateStyle) -> StateStyle) -> Self {
        self.error
            .refine(&build(StateStyle::default()).into_refinement());
        self
    }
}

impl<M: InputModeKind> Styled for Field<M> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<M: InputModeKind> ParentElement for Field<M> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<M: InputModeKind> RenderOnce for Field<M> {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let description = match (&self.error, &self.description) {
            (Some(error), Some(help)) => Some(format!("{error} {help}").into()),
            (Some(error), None) => Some(error.clone()),
            (None, help) => help.clone(),
        };
        self.input.update(cx, |state, _| {
            let accessibility = state.accessibility().cloned().unwrap_or_else(|| {
                InputAccessibility::new(Some(if M::MULTI_LINE {
                    Role::MultilineTextInput
                } else {
                    Role::TextInput
                }))
            });
            state.set_accessibility(
                accessibility
                    .with_label(Some(self.label.clone()))
                    .with_description(description)
                    .with_required(self.required)
                    .with_invalid(self.error.is_some()),
            );
        });

        let disabled = self.input.read(cx).presentation().is_disabled();
        let label_input = self.input;
        let label = div()
            .id((self.id.clone(), "label"))
            .test_support()
            .role(Role::Label)
            .aria_label(self.label.clone())
            .refine_style(&self.styles.label)
            .child(self.label)
            .when(disabled, |label| {
                label.on_mouse_down(MouseButton::Left, |_, _, cx| {
                    cx.stop_propagation();
                })
            })
            .when(!disabled, |label| {
                label.on_click(move |_, window, cx| {
                    label_input.update(cx, |state, cx| {
                        if !state.presentation().is_disabled() {
                            state.focus(window, cx);
                        }
                    });
                    cx.stop_propagation();
                })
            })
            .a11y_disabled(disabled);

        self.base
            .role(Role::Group)
            .child(label)
            .children(self.children)
            .when_some(self.description, |this, description| {
                this.child(
                    div()
                        .id((self.id.clone(), "description"))
                        .test_support()
                        .role(Role::Label)
                        .aria_label(description.clone())
                        .refine_style(&self.styles.description)
                        .child(description),
                )
            })
            .when_some(self.error, |this, error| {
                this.child(
                    div()
                        .id((self.id, "error"))
                        .test_support()
                        .role(Role::Label)
                        .aria_label(error.clone())
                        .refine_style(&self.styles.error)
                        .child(error),
                )
            })
    }
}

/// Native metadata a [`ControlField`] hands to its non-text control.
///
/// Checkbox, Switch, RadioGroup and Select accept it through `.field(...)`,
/// which names the control, describes it with the error then help text, and
/// marks it required or invalid on its existing accessibility node.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FieldMetadata {
    pub label: Option<SharedString>,
    pub description: Option<SharedString>,
    pub required: bool,
    pub invalid: bool,
}

/// An unstyled field for a checkbox, switch, radio group, select or other
/// non-text control.
///
/// It renders the label, the control built from [`FieldMetadata`], help and an
/// optional error. Clicking the label focuses `focus` without adding a Tab stop,
/// then runs the optional label activation, for example toggling a checkbox as
/// an HTML `label` does. Validation and layout belong to the application; use
/// [`Field`] for Input and Textarea.
#[derive(IntoElement)]
pub struct ControlField {
    id: ElementId,
    base: crate::ObservedElement<gpui::Stateful<Div>>,
    focus: FocusHandle,
    label: SharedString,
    description: Option<SharedString>,
    error: Option<SharedString>,
    required: bool,
    disabled: bool,
    control_first: bool,
    activate: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
    control: Option<Box<dyn FnOnce(FieldMetadata) -> AnyElement>>,
    styles: FieldStyles,
}

impl ControlField {
    pub fn new(
        id: impl Into<ElementId>,
        focus: &FocusHandle,
        label: impl Into<SharedString>,
    ) -> Self {
        let id = id.into();
        Self {
            base: div().id(id.clone()).test_support(),
            id,
            focus: focus.clone(),
            label: label.into(),
            description: None,
            error: None,
            required: false,
            disabled: false,
            control_first: false,
            activate: None,
            control: None,
            styles: FieldStyles::default(),
        }
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Supplies an application validation error; `None` clears invalid state.
    pub fn error(mut self, error: Option<SharedString>) -> Self {
        self.error = error;
        self
    }

    /// Reports the requirement to accessibility clients; does not reject values.
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Mirrors the control's disabled state so the label neither focuses nor
    /// activates it.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Renders the control before its label, as for a checkbox or switch.
    pub fn control_first(mut self, control_first: bool) -> Self {
        self.control_first = control_first;
        self
    }

    /// Runs after a label click focuses the control.
    pub fn on_label_activate(mut self, activate: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.activate = Some(Rc::new(activate));
        self
    }

    /// Builds the control from this field's metadata; pass it to `.field(...)`.
    pub fn control<E: IntoElement>(
        mut self,
        build: impl FnOnce(FieldMetadata) -> E + 'static,
    ) -> Self {
        self.control = Some(Box::new(move |metadata| build(metadata).into_any_element()));
        self
    }

    pub fn styles(mut self, build: impl FnOnce(FieldStyles) -> FieldStyles) -> Self {
        self.styles = build(self.styles);
        self
    }
}

impl Styled for ControlField {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for ControlField {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let metadata = FieldMetadata {
            label: Some(self.label.clone()),
            description: match (&self.error, &self.description) {
                (Some(error), Some(help)) => Some(format!("{error} {help}").into()),
                (Some(error), None) => Some(error.clone()),
                (None, help) => help.clone(),
            },
            required: self.required,
            invalid: self.error.is_some(),
        };
        let control = self.control.map(|build| build(metadata));
        let disabled = self.disabled;
        let focus = self.focus;
        let activate = self.activate;
        let label = div()
            .id((self.id.clone(), "label"))
            .test_support()
            .role(Role::Label)
            .aria_label(self.label.clone())
            .refine_style(&self.styles.label)
            .child(self.label)
            .when(disabled, |label| {
                label.on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            })
            .when(!disabled, |label| {
                label.on_click(move |_, window, cx| {
                    focus.focus(window, cx);
                    if let Some(activate) = &activate {
                        activate(window, cx);
                    }
                    cx.stop_propagation();
                })
            })
            .a11y_disabled(disabled);
        let (first, second): (AnyElement, Option<AnyElement>) = if self.control_first {
            match control {
                Some(control) => (control, Some(label.into_any_element())),
                None => (label.into_any_element(), None),
            }
        } else {
            (label.into_any_element(), control)
        };
        self.base
            .role(Role::Group)
            .child(first)
            .children(second)
            .when_some(self.description, |this, description| {
                this.child(
                    div()
                        .id((self.id.clone(), "description"))
                        .test_support()
                        .role(Role::Label)
                        .aria_label(description.clone())
                        .refine_style(&self.styles.description)
                        .child(description),
                )
            })
            .when_some(self.error, |this, error| {
                this.child(
                    div()
                        .id((self.id, "error"))
                        .test_support()
                        .role(Role::Label)
                        .aria_label(error.clone())
                        .refine_style(&self.styles.error)
                        .child(error),
                )
            })
    }
}

#[cfg(test)]
mod tests {
    use gpui::{
        AppContext as _, Context, Element as _, FocusHandle, Modifiers, Render, TestAppContext,
        accesskit, point, px,
    };

    use super::*;
    use crate::InputState;

    #[gpui::test]
    fn feedback_preserves_the_native_inputs_existing_capabilities(cx: &mut TestAppContext) {
        cx.update(crate::init);
        let window = cx.add_empty_window();
        window.update(|window, cx| {
            let input = cx.new(|cx| {
                let mut input = InputState::new(window, cx).default_value("private value");
                input.set_accessibility(
                    InputAccessibility::new(Some(Role::ComboBox))
                        .with_placeholder(Some("Find a session".into()))
                        .with_accessibility_id(Some("session-editor".into()))
                        .with_expanded(Some(true))
                        .with_value_exposed(false),
                );
                input
            });

            for invalid in [true, false] {
                let _ = Field::new("session", &input, "Session name")
                    .description("Choose a recognizable name.")
                    .required(true)
                    .error(invalid.then(|| "A name is required.".into()))
                    .render(window, cx);

                input.update(cx, |state, cx| {
                    let element = state.render(window, cx).into_element();
                    assert_eq!(element.a11y_role(), Some(Role::ComboBox));
                    let mut node = accesskit::Node::new(Role::ComboBox);
                    element.write_a11y_info(&mut node);
                    assert_eq!(node.label(), Some("Session name"));
                    assert_eq!(
                        node.description(),
                        Some(if invalid {
                            "A name is required. Choose a recognizable name."
                        } else {
                            "Choose a recognizable name."
                        }),
                    );
                    assert_eq!(node.placeholder(), Some("Find a session"));
                    assert_eq!(node.author_id(), Some("session-editor"));
                    assert_eq!(node.is_expanded(), Some(true));
                    assert!(node.is_required());
                    assert_eq!(node.invalid(), invalid.then_some(accesskit::Invalid::True));
                    assert_eq!(node.value(), None, "field must preserve hidden values");
                    assert_eq!(state.value(), "private value");
                });
            }

            input.update(cx, |state, cx| state.set_disabled(true, cx));
            let _ = Field::new("session", &input, "Session name").render(window, cx);
            input.update(cx, |state, cx| {
                let element = state.render(window, cx).into_element();
                let mut node = accesskit::Node::new(Role::ComboBox);
                element.write_a11y_info(&mut node);
                assert!(node.is_disabled());
                assert!(!node.is_required());
                assert_eq!(node.invalid(), None);
                assert_eq!(node.description(), None);
            });
        });
    }

    #[gpui::test]
    fn label_disabled_state_blocks_parent_focus_but_readonly_remains_focusable(
        cx: &mut TestAppContext,
    ) {
        struct Harness {
            input: Entity<InputState>,
            outside: FocusHandle,
            parent: FocusHandle,
        }
        impl Render for Harness {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                div()
                    .flex()
                    .flex_col()
                    .w(px(200.))
                    .child(
                        div().id("parent").track_focus(&self.parent).child(
                            Field::new("field", &self.input, "Name")
                                .flex()
                                .flex_col()
                                .styles(|styles| styles.label(|style| style.h(px(32.))))
                                .child(
                                    crate::InputBase::new("input")
                                        .h(px(32.))
                                        .child(crate::Input::new(&self.input)),
                                ),
                        ),
                    )
                    .child(
                        crate::Button::new("outside")
                            .track_focus(&self.outside)
                            .h(px(32.))
                            .child("Outside"),
                    )
            }
        }
        cx.update(crate::init);
        let (root, cx) = cx.add_window_view(|window, cx| {
            let view = cx.new(|cx| Harness {
                input: cx.new(|cx| InputState::new(window, cx).default_value("Name")),
                outside: cx.focus_handle(),
                parent: cx.focus_handle(),
            });
            crate::Root::new(view, window, cx)
        });
        let view = root
            .read_with(cx, |root, _| root.view().clone())
            .downcast::<Harness>()
            .unwrap();
        for (disabled, readonly) in [(true, false), (false, true), (false, false)] {
            view.update(cx, |view, cx| {
                view.input.update(cx, |input, cx| {
                    input.set_disabled(disabled, cx);
                    input.set_readonly(readonly, cx);
                });
                cx.notify();
            });
            cx.update(|window, cx| {
                window.activate_window();
                view.read(cx).outside.clone().focus(window, cx);
                window.draw(cx).clear(cx);
            });
            cx.simulate_click(point(px(20.), px(16.)), Modifiers::default());
            cx.update(|window, cx| {
                window.draw(cx).clear(cx);
                let view = view.read(cx);
                if disabled {
                    assert!(
                        view.outside.is_focused(window),
                        "a disabled field label must not focus its parent"
                    );
                } else {
                    assert!(
                        view.input
                            .read(cx)
                            .presentation()
                            .focus_handle()
                            .is_focused(window)
                    );
                }
                #[cfg(feature = "test-support")]
                {
                    let label = crate::test_support::find(
                        window,
                        &[],
                        &(ElementId::from("field"), "label").into(),
                    )
                    .unwrap();
                    assert_eq!(label.disabled(), disabled.then_some(true));
                    assert_eq!(label.focused(), None);
                }
            });
        }
    }

    #[gpui::test]
    fn control_fields_describe_choice_controls_and_forward_label_clicks(cx: &mut TestAppContext) {
        use crate::{Checkbox, Root, SelectItem, SelectState, Switch};
        use std::cell::Cell;

        cx.update(crate::init);
        let window = cx.add_empty_window();
        window.update(|window, cx| {
            let metadata = FieldMetadata {
                label: Some("Share recordings".into()),
                description: Some("Pick one. Viewers can comment.".into()),
                required: true,
                invalid: true,
            };
            let select = cx.new(|cx| SelectState::new([SelectItem::new("team", "Team")], cx));
            let radio = cx
                .new(|cx| crate::RadioGroupState::new([crate::RadioItem::new("team", "Team")], cx));
            fn check(element: impl IntoElement, role: Role) {
                let element = element.into_element();
                let mut node = accesskit::Node::new(role);
                element.write_a11y_info(&mut node);
                assert_eq!(node.label(), Some("Share recordings"), "{role:?}");
                assert_eq!(node.description(), Some("Pick one. Viewers can comment."));
                assert!(node.is_required());
                assert_eq!(node.invalid(), Some(accesskit::Invalid::True));
            }
            check(
                Checkbox::new("share")
                    .track_focus(&cx.focus_handle())
                    .accessibility_label("Ignored")
                    .field(metadata.clone())
                    .render(window, cx),
                Role::CheckBox,
            );
            check(
                Switch::new("share-switch")
                    .track_focus(&cx.focus_handle())
                    .field(metadata.clone())
                    .render(window, cx),
                Role::Switch,
            );
            check(
                crate::RadioGroup::new("share-radio", &radio)
                    .field(metadata.clone())
                    .render(window, cx),
                Role::RadioGroup,
            );
            check(
                crate::Select::new("share-select", &select)
                    .field(metadata.clone())
                    .render(window, cx),
                Role::ComboBox,
            );
        });

        struct Form {
            focus: FocusHandle,
            checked: Rc<Cell<bool>>,
            disabled: bool,
        }
        impl Render for Form {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                let checked = self.checked.clone();
                let toggle = self.checked.clone();
                let focus = self.focus.clone();
                ControlField::new("notify-field", &self.focus, "Email me")
                    .description("Sends one summary a day.")
                    .disabled(self.disabled)
                    .control_first(true)
                    .on_label_activate(move |_, _| toggle.set(!toggle.get()))
                    .control(move |metadata| {
                        Checkbox::new("notify")
                            .track_focus(&focus)
                            .checked(checked.get())
                            .field(metadata)
                            .w(px(16.))
                            .h(px(16.))
                    })
                    .flex()
                    .gap(px(8.))
            }
        }
        let (root, cx) = cx.add_window_view(|window, cx| {
            let form = cx.new(|cx| Form {
                focus: cx.focus_handle(),
                checked: Rc::default(),
                disabled: false,
            });
            Root::new(form, window, cx)
        });
        let form = root
            .read_with(cx, |root, _| root.view().clone())
            .downcast::<Form>()
            .unwrap();
        cx.update(|window, cx| {
            window.activate_window();
            window.draw(cx).clear(cx);
        });
        let label = cx.update(|window, _| {
            let label = crate::test_support::find(
                window,
                &[],
                &(ElementId::from("notify-field"), "label").into(),
            )
            .unwrap();
            let control = crate::test_support::find(window, &[], &"notify".into()).unwrap();
            assert!(
                control.bounds().right() <= label.bounds().left(),
                "control first"
            );
            label.bounds().center()
        });
        cx.simulate_click(label, Modifiers::default());
        cx.update(|window, cx| {
            let form = form.read(cx);
            assert!(
                form.focus.is_focused(window),
                "the label focuses the control"
            );
            assert!(form.checked.get(), "and activates it");
        });
        form.update(cx, |form, cx| {
            form.disabled = true;
            cx.notify();
        });
        cx.update(|window, cx| {
            window.draw(cx).clear(cx);
            window.blur(cx);
        });
        cx.simulate_click(label, Modifiers::default());
        cx.update(|window, cx| {
            let form = form.read(cx);
            assert!(!form.focus.is_focused(window));
            assert!(form.checked.get(), "a disabled field ignores label clicks");
        });
    }
}
