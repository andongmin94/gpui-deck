pub(crate) mod a11y_current;
pub(crate) mod a11y_disabled;
pub(crate) mod accordion;
pub mod actions;
pub(crate) mod alert_dialog;
pub(crate) mod animation;
pub(crate) mod auto_scroll;
pub(crate) mod breadcrumb;
pub(crate) mod button;
pub(crate) mod calendar;
pub(crate) mod carousel;
pub(crate) mod chart;
pub(crate) mod checkbox;
pub(crate) mod collapsible;
pub(crate) mod combobox;
pub(crate) mod command;
pub(crate) mod component_traits;
pub(crate) mod context_menu;
pub(crate) mod date_picker;
pub(crate) mod dialog;
pub(crate) mod direction;
pub(crate) mod drawer;
pub(crate) mod dropdown_menu;
pub(crate) mod element_ext;
pub(crate) mod event;
pub(crate) mod field;
pub(crate) mod focus_trap;
pub(crate) mod geometry;
pub(crate) mod global_state;
pub(crate) mod hover_card;
pub mod input;
pub(crate) mod input_otp;
pub(crate) mod label;
pub(crate) mod long_press;
#[cfg(all(target_os = "macos", not(test)))]
pub(crate) mod macos_accessibility;
pub(crate) mod menu;
pub(crate) mod menubar;
pub(crate) mod message_scroller;
pub(crate) mod navigation_menu;
pub(crate) mod observe;
pub(crate) mod overscroll;
pub(crate) mod popover;
pub(crate) mod popup;
pub(crate) mod positioner;
pub(crate) mod progress;
pub(crate) mod questionnaire;
pub(crate) mod radio_group;
pub(crate) mod reduce_motion;
pub(crate) mod resizable;
pub(crate) mod root;
pub(crate) mod scroll_area;
pub(crate) mod scroll_bounce;
pub(crate) mod scrollbar;
pub(crate) mod select;
pub(crate) mod separator;
pub(crate) mod slider;
pub(crate) mod state_style;
pub(crate) mod styled;
pub(crate) mod switch;
pub(crate) mod table;
pub(crate) mod tabs;
pub(crate) mod text_boundary;
pub(crate) mod theme;
pub(crate) mod toast;
pub(crate) mod toggle;
pub(crate) mod toggle_group;
pub(crate) mod tooltip;
pub(crate) mod touch_selection;

pub use accordion::{
    Accordion, AccordionEvent, AccordionHeader, AccordionItem, AccordionPanel, AccordionState,
};
pub use alert_dialog::AlertDialog;
pub(crate) use auto_scroll::AutoScroll;
pub use breadcrumb::{Breadcrumb, BreadcrumbItem, BreadcrumbList};
pub use button::{Button, ButtonStyles};
pub use calendar::{
    Calendar, CalendarDay, CalendarEvent, CalendarSelection, CalendarState, Date, Weekday,
};
pub use carousel::{Carousel, CarouselControl, CarouselEvent, CarouselSlide, CarouselState};
pub use chart::{Chart, ChartEvent, ChartLayout, ChartSeries, ChartState};
pub use checkbox::{Checkbox, CheckboxState, CheckboxStyles};
pub use collapsible::{
    Collapsible, CollapsibleContent, CollapsibleEvent, CollapsibleState, CollapsibleTrigger,
};
pub use combobox::{
    Combobox, ComboboxEvent, ComboboxGroup, ComboboxInput, ComboboxList, ComboboxOption,
    ComboboxState,
};
pub use command::{
    Command, CommandEvent, CommandGroup, CommandInput, CommandList, CommandOption, CommandState,
    SearchItem,
};
pub use component_traits::Selectable;
pub use context_menu::ContextMenu;
pub use date_picker::{DatePicker, DatePickerEvent, DatePickerState, parse_date_text};
pub use dialog::{
    Dialog, DialogBackdrop, DialogChangeReason, DialogClose, DialogDescription, DialogEvent,
    DialogPopup, DialogState, DialogTitle,
};
pub use direction::{Direction, DirectionProvider};
pub use drawer::{Drawer, DrawerState};
pub use dropdown_menu::DropdownMenu;
pub use element_ext::ElementExt;
pub use event::{InteractiveElementExt, OngoingScrollExt};
pub use field::{ControlField, Field, FieldMetadata, FieldStyles};
pub use focus_trap::FocusTrapElement;
pub(crate) use focus_trap::active_focus_trap;
pub use geometry::*;
pub(crate) use global_state::GlobalState;
pub use hover_card::{HoverCard, HoverCardState};
pub use input::{
    Input, InputAccessibility, InputBase, InputEditorStyle, InputState, InputStyles, Textarea,
    TextareaState,
};
pub use input_otp::{InputOtp, InputOtpEvent, InputOtpSlot, InputOtpState};
pub use label::Label;
#[cfg(all(target_os = "macos", not(test)))]
pub(crate) use macos_accessibility::install_window_hit_test_forwarder;
pub use menu::{MenuEvent, MenuItem, MenuOption, MenuState, MenuSubmenu};
pub use menubar::{Menubar, MenubarItem, MenubarMenu, MenubarState};
pub use message_scroller::{MessageScroller, MessageScrollerState};
pub use navigation_menu::{
    NavigationMenu, NavigationMenuEntry, NavigationMenuEvent, NavigationMenuItem,
    NavigationMenuLink, NavigationMenuState,
};
pub use observe::{ObservedElement, ObservedTextElement, TestSupportExt, TestTextExt};
pub use popover::{Popover, PopoverState};
pub use popup::{POPUP_PRIORITY, Popup};
pub use positioner::{Align, Positioner, ResolvedPosition};
pub use progress::Progress;
pub use questionnaire::{Questionnaire, QuestionnaireStage, QuestionnaireState};
pub use radio_group::{RadioGroup, RadioGroupEvent, RadioGroupState, RadioItem, RadioOption};
pub use reduce_motion::{
    ReduceMotionPreference, apply_system_reduce_motion, set_reduce_motion_preference,
};
pub use resizable::{Resizable, ResizableEvent, ResizableHandle, ResizablePanel, ResizableState};
pub use root::Root;
pub use scroll_area::ScrollArea;
pub use scroll_bounce::{ScrollBounce, ScrollBounceMotion};
pub use scrollbar::{
    Scrollbar, ScrollbarAxis, ScrollbarEntrance, ScrollbarHandle, ScrollbarMode, ScrollbarMotion,
    ScrollbarStyles, ScrollbarThumbStyle, ScrollbarTrackStyle,
};
pub use select::{Select, SelectEvent, SelectItem, SelectOption, SelectState};
pub use separator::Separator;
pub use slider::{Slider, SliderEvent, SliderState, SliderThumb};
pub use state_style::StateStyle;
#[cfg(any(feature = "inspector", debug_assertions))]
pub use styled::styled_ext_reflection_methods;
pub use styled::{RoleOverride, StyledExt, box_shadow, h_flex, v_flex};
pub use switch::{Switch, SwitchStyles};
pub use table::{Table, TableCaption, TableCell, TableColumn, TableRow};
pub use tabs::{Tab, TabItem, TabList, TabPanel, TabsEvent, TabsState};
pub use theme::{InputColors, ScrollbarTheme, Theme};
pub use toast::{
    Toast, ToastClose, ToastDismissReason, ToastEvent, ToastKey, ToastState, Toaster, ToasterState,
};
pub use toggle::Toggle;
pub use toggle_group::{ToggleGroup, ToggleGroupEvent, ToggleGroupState, ToggleItem, ToggleOption};
pub use tooltip::{Tooltip, TooltipState};
pub use touch_selection::{SelectionEdge, TouchHandle, TouchSelectionSnapshot};

use gpui::App;

/// Whether the application is compiled for iOS or Android.
#[inline]
pub const fn is_mobile() -> bool {
    cfg!(any(target_os = "ios", target_os = "android"))
}

/// Register keyboard actions and shared input/focus state before creating views.
pub fn init(cx: &mut App) {
    let _ = Theme::global_mut(cx);
    GlobalState::init(cx);
    reduce_motion::init(cx);
    accordion::init(cx);
    calendar::init(cx);
    carousel::init(cx);
    chart::init(cx);
    context_menu::init(cx);
    dialog::init(cx);
    dropdown_menu::init(cx);
    focus_trap::init(cx);
    menu::init(cx);
    menubar::init(cx);
    navigation_menu::init(cx);
    popover::init(cx);
    radio_group::init(cx);
    resizable::init(cx);
    select::init(cx);
    slider::init(cx);
    tabs::init(cx);
    toast::init(cx);
    toggle_group::init(cx);
    input::init(cx);
    root::init(cx);
}

#[cfg(feature = "test-support")]
pub mod test_support;
