//! Touch long-press recognition for a parent element's bounds.

use gpui::{
    App, Entity, HitboxBehavior, IntoElement, LongPressEvent, Pixels, Point, Styled as _,
    TouchPhase, Window, canvas,
};

/// An absolute overlay that reports a touch long press starting inside its
/// parent. GPUI's gesture recognizer owns timing and slop. `on_start` returns
/// whether it handled the press; a handled press is captured for `owner`, so
/// its later moves and release do not pan or reach other elements.
///
/// The overlay's hitbox does not block other hitboxes, so ordinary pointer
/// handling of the parent and its children is unchanged.
pub(crate) fn long_press_region<T: 'static>(
    owner: Entity<T>,
    on_start: impl Fn(Point<Pixels>, &mut Window, &mut App) -> bool + 'static,
) -> impl IntoElement {
    canvas(
        |bounds, window, _| window.insert_hitbox(bounds, HitboxBehavior::Normal),
        move |_, hitbox, window, _| {
            window.on_mouse_event(move |event: &LongPressEvent, phase, window, cx| {
                if !phase.bubble() {
                    return;
                }
                if event.phase == TouchPhase::Started {
                    if window.default_prevented()
                        || !hitbox.is_hovered(window)
                        || !on_start(event.start_position, window, cx)
                    {
                        return;
                    }
                    window.capture_long_press(&owner);
                } else if !window.has_long_press_capture(&owner) {
                    return;
                }
                window.prevent_default();
                cx.stop_propagation();
            });
        },
    )
    .absolute()
    .top_0()
    .left_0()
    .size_full()
}
