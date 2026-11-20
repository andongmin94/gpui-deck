# Layout and data

Table, ScrollArea, and Resizable provide reusable structure and native behavior. Applications
own their data, layout dimensions, colors, typography, and cell contents. The
[two-style example](../crates/base/examples/two_styles.rs) combines them in its
Schedule tab and uses ScrollArea for the surrounding form.

## Table

A Table defines columns once and applies them to its header, body, and footer.
Each row must provide exactly one cell per column. Use fixed widths for stable
tracks and positive flexible weights to divide the remaining width:

```rust
Table::new(
    "sessions",
    [TableColumn::fixed(px(180.)), TableColumn::flex(1.)],
)
.accessibility_label("Sessions")
.w_full()
.header(
    TableRow::new("sessions-header")
        .cell(TableCell::new("session-heading").p_2().child("Session"))
        .cell(TableCell::new("date-heading").p_2().child("Date")),
)
.row(
    TableRow::new("session-1")
        .cell(TableCell::new("session-1-name").row_header(true).p_2().child("Workshop"))
        .cell(TableCell::new("session-1-date").p_2().child("2026-11-09")),
)
.caption(TableCaption::new("sessions-caption").child("Scheduled sessions"))
```

`header` and `footer` accept one row each. Append body rows through `row` or
`rows`; `TableRow::cells` also accepts an iterator. An empty body is valid, so an
application can render its empty-state explanation alongside the table.

Every part implements `Styled`. Cells and captions accept arbitrary children;
an application can put an existing Button or Checkbox in a cell. Structural
table parts add no Tab stops and do not intercept those controls' activation.
Table owns the row's horizontal layout. Apply spacing inside cells rather than
adding horizontal row padding, margins, or gaps. Cell styles apply to an inner
content surface, preserving the shared outer column width even when header and
body padding differ.

| Part | Native semantics |
| --- | --- |
| Table | Table, row count, column count, and an explicit accessible label |
| Header/body/footer | RowGroup |
| Row | Row and its zero-based index |
| Header cell | ColumnHeader with row/column indices |
| Ordinary cell | Cell with row/column indices |
| Cell with `row_header(true)` | RowHeader, unless it belongs to the column-header row |
| Caption | Caption without a focus action |

Indices include the header and footer. A visible caption does not automatically
name the table; supply `accessibility_label` separately. Cells also accept an
explicit accessible label when their visible content does not provide one.

This is an ordinary table, without cell navigation or a data controller. Row or
column spans, automatic intrinsic column sizing, sorting, filtering, pagination,
selection management, and virtualization are not supplied. `TableRow::selected`
and `TableCell::sort_direction` project application-owned selection and sorting
into native metadata; they do not perform data operations. Data operations should be
composed over this structure when a real application needs them.

The copyable [DataTable recipe](presentation-recipes.md#data-table) adds text
filtering, single-column sorting, pagination, page/row checkbox selection, column
visibility, empty-state recovery, and stable-ID row actions. The native example uses it to browse and apply recording presets;
its query and page state belong to the application, outside Table.

## ScrollArea

Retain a GPUI ScrollHandle in the view and give the viewport a bounded size on
its scrolling axes. The same handle drives native wheel/trackpad scrolling and
the existing overlay scrollbar:

```rust
// Store once in the view, not on each render.
let scroll = gpui::ScrollHandle::new();

ScrollArea::new("session-list", &scroll)
    .accessibility_label("Sessions")
    .h(px(240.))
    .w_full()
    .content_styles(|style| style.flex().flex_col())
    .children(rows)
```

The default axis is vertical. Use `.axis(ScrollbarAxis::Horizontal)` or `Both`
for other content. Give overflowing content a minimum/fixed extent or prevent
it from shrinking when appropriate. Ordinary GPUI styles size and decorate the
outer frame. Use `content_styles` for the viewport's child layout, spacing, and
padding. The viewport and overlay scrollbar use GPUI's normal rendering so
ancestor visibility and opacity apply to both.

`scrollbar_mode` accepts the existing Scrolling, Hover, or Always modes.
`scrollbar_styles` uses the existing typed track/thumb styles. The bar overlays
content and reserves no gutter, so leave application padding if its content
should not overlap the bar. For example, the plan table uses bottom/right
padding and always-visible scrollbars for its resizable viewport.

The public frame exposes the native ScrollView role and accepts an accessible label.
It adds no independent focus stop or key bindings: input arrows keep editing
text, and Calendar arrows keep navigating dates. Plain static content can be
scrolled with the pointer or the retained handle; a separate keyboard scrolling
control is not part of this contract.

Descendants can use GPUI's native `request_autoscroll` during prepaint to request
reveal. Input, Textarea, Calendar, and newly focused Button controls issue requests
when needed.
ScrollArea clamps the offset on its enabled axes and requests a new frame when
the viewport needs to move. A nested area's adjusted request continues to its
ancestors, and a sibling's pending request is preserved rather than used to
scroll the wrong viewport. Offsets on disabled axes are cleared.

ScrollArea reuses GPUI's ordinary scrolling and the retained Scrollbar pointer
implementation. It does not virtualize children, add elastic scrolling, or
anchor streaming/transcript content.

## A table inside a scroll area

Set the table's width or minimum width explicitly when columns should exceed
the viewport. The native example uses three fixed columns totaling 490 pixels
inside a ScrollArea supporting both axes. Its surrounding settings form scrolls vertically.
The nested table can move horizontally without altering the form's date or
scrolling unrelated siblings. Date selection and name/preset edits update the
table from the same retained state used by the other controls.

## Resizable

Resizable divides a bounded frame into two or more continuously laid out
panels separated by focusable dividers. Its state owns panel sizes, each
panel's constraints, and one focus handle per divider:

```rust
let split = cx.new(|cx| {
    ResizableState::new(
        Axis::Horizontal,
        [
            ResizablePanel::new(0.25).limits(0.15..=0.4).collapsible(0.),
            ResizablePanel::new(0.5).limits(0.3..=1.),
            ResizablePanel::new(0.25).limits(0.15..=0.4),
        ],
        cx,
    )
});

Resizable::new("workspace", &split)
    .size_full()
    .panel(sidebar)
    .panel(editor)
    .panel(inspector)
    .handle(|index, handle| {
        handle
            .accessibility_label(["Sidebar width", "Editor width"][index])
            .w(px(6.))
            .bg(divider_color)
    })
```

Supply one `panel` per configured panel, in order. `handle` builds every
divider from its index, which is the index of the panel before it.
`Axis::Horizontal` places panels left to right and uses the handle's width;
`Axis::Vertical` places them top to bottom and uses its height. Bound the frame's
dimensions and give handles an actual thickness. Sizes are fractions of the
usable space after divider thicknesses, sum to one, and stay proportional when
the parent resizes. Frame padding is supported; the internal layout has no gap.

`ResizablePanel::new(default)` starts free between zero and one. `limits`
bounds the expanded size and `collapsible(size)` allows a smaller collapsed
size. Constructing a state panics when defaults violate a panel's constraints
or do not sum to one. `pixel_limits(min..=max)` adds logical-pixel bounds that
apply once the group is measured; the tighter of the fractional and pixel bound
wins. When the measured space changes, a layout that breaks the new bounds is
refitted once, nearest first, and emits a Change; a still-valid layout keeps its
proportions. If the space cannot satisfy every pixel limit, the group uses the
fractional limits alone until it can. Divider values and Home/End reflect the
measured bounds.

Moving a divider grows the panel on one side and shrinks the panels on the
other side nearest first, so a panel at its minimum passes the move to the next
one. A move no panel can absorb leaves the divider in place. Dragging past
halfway between a collapsible panel's minimum and collapsed size snaps it
closed; dragging back past that point reopens it at its minimum.

| Input | Result |
| --- | --- |
| Drag a divider | Resize from the actual press position, cascading within constraints |
| Left / Right in a horizontal group | Shrink / grow the panel before the divider by two percentage points |
| Up / Down in a vertical group | Shrink / grow the panel before the divider by two percentage points |
| Arrow toward a collapsed panel | Reopen it at its minimum |
| Arrow against a collapsible panel at its minimum | Collapse it |
| Home / End | Give the panel before the divider its smallest / largest allowed size |
| Enter | Collapse a collapsible panel before the divider, or restore its last size |
| Tab / Shift+Tab | Traverse dividers and panel controls in ordinary document order |

Home, End and Enter follow the
[APG window splitter pattern](https://www.w3.org/WAI/ARIA/apg/patterns/windowsplitter/).
Only dividers own the resize key context. Text editing, selection, and scrolling
inside panels retain their own behavior. Pointer dragging uses GPUI's drag
lifecycle and identifies its owning state and divider, so independent or nested
groups do not react to another divider's drag.

Each divider has a native Splitter role whose numeric value, limits, and step
describe the panel before it, in percent. Native orientation describes the
divider line. Give each divider a label naming the panel it adjusts and keep its
grip children non-interactive. The application supplies all styling.

A panel collapsed to zero size unmounts its content, so it contributes no focus
stops or accessibility nodes. If focus was inside, it moves to the adjacent
divider (or the group itself while resizing is disabled). A non-zero collapsed
size, such as an icon rail, keeps its content mounted.

Observe the state to update presentation. `sizes()`, `panel_count()`,
`is_collapsed(index)`, `axis()`, `handle_focus(index)`, and `is_disabled()`
expose its contract. `set_sizes` replaces the whole layout and panics on an
invalid one, then fits it to the measured pixel limits. To persist a layout,
store `sizes()` from `ResizableEvent::Change` in application storage and pass
it to `set_sizes` on the next launch.
`collapse(index)` and `expand(index)` act on collapsible panels and do nothing
when the neighbors cannot absorb the change. Each layout change emits one
`ResizableEvent::Change(sizes)`, including programmatic updates. `set_disabled`
blocks pointer, keyboard, and accessibility resizing and normal Tab entry; a
focused divider retains its binding so Tab can leave.

The Schedule example puts the shared notes in a collapsible first panel and the
plan table in a two-axis ScrollArea in the second, which keeps at least 96 px.
Closing or switching tabs
preserves text and sizes because the application retains their states. The
library writes no files; the storage location belongs to the application.

## Carousel

`CarouselState::new(labels, cx)` retains a fixed ordered collection and selected
(leading) slide. Selection is bounded by default; `.looping(true)` permits wrapping.
`.slides_per_view(n)` or `set_slides_per_view(n, window, cx)` shows `n`
consecutive slides, like shadcn's fractional item basis. Each step moves by one
slide and the last view stays full, so `snap_count()` leading positions exist and
`visible_range()` names the mounted slides; looping wraps between the last full
view and the first. `Carousel::new(id, &state, label)` supplies a named focusable
region, and `CarouselSlide::new(&state, index)` mounts only slides in the current
view. The application retains slide data and supplies media, slide widths (for
example equal `flex_1` slides in a row), content and indicator presentation.
`CarouselControl::previous/next` compose Button with boundary disabled behavior.

Left/Right act while the horizontal region itself holds focus, Up/Down for a
vertical region; Home/End select the first and last views. Slide descendants keep
their own keys. Configure axis and direction on `Carousel`; every slide uses that
same state for swipes. Direction is inherited by default. A pointer drag of 40
logical pixels snaps once per gesture, including a single move followed
immediately by release. Moving slides out of view, including by shrinking the
view, returns focus inside them to the carousel scope before unmounting; focus in
slides that stay visible and on external previous/next controls is kept. There is
no inertia, animated motion or free-scrolling between snaps.

`set_autoplay(Some(interval), window, cx)` advances one slide per interval and
wraps from the last view to the first; `None` or `is_autoplaying()` control and
report it. Rotation pauses while the pointer is over the carousel, focus is inside
it, the window is inactive, or reduced motion is on, and keyboard, swipe and
`CarouselControl` navigation stop it, like Embla's default autoplay plugin.
Application calls such as pagination `select` do not stop it. Render a visible
play/pause Button, as the catalogue does with "Play slideshow", so people can
stop rotation (WCAG 2.2.2).

## Chart

`ChartState` accepts category labels and `ChartSeries` values. Each series has one
finite value per category. The retained state supplies visible series, inspected
category, exact-value descriptions, and a zero-inclusive signed numeric range.
Hiding all series or using all-zero data yields a usable `0..1` range. The range
is retained until visibility or layout changes; normalizing a point does not
rescan the dataset.

`ChartLayout` selects how the same data shares the plot, and `set_layout` keeps
visibility and inspection:

- `Grouped` (default) places side-by-side series over evenly spaced category bands.
- `Stacked` stacks positive and negative values separately from zero. Its range
  covers each category's visible sums, and `stack_span(series, category)` gives a
  visible segment's start and end values.
- `Pie` gives each category one slice sized by its nonnegative visible-series
  total. `pie_slices` returns start/end turns; a zero or negative total has an
  empty slice.
- `Radar` places categories on evenly spaced spokes (`radar_turn`) with radii
  normalized like Grouped.

Radial turns run clockwise from the top and mirror counterclockwise in RTL.
`Chart` provides a named plot focus region: Left/Right/Home/End inspect
categories in every layout. Pointer travel inspects by horizontal band for
Grouped/Stacked, and by angle inside the largest centered circle for Pie/Radar;
a pointer outside that circle or over no slice keeps the current inspection. It
emits inspection/visibility events without prescribing colors or draw paths.

The copyable `examples/recipes/chart.rs` builds on this behavior with GPUI canvas
line, grouped bar, area, stacked bar/area, pie, donut and radar plots, axis or
spoke labels, a pie category key, Toggle-based series legend, an exact-value
readout and an ordinary Table containing the same complete dataset. `ChartKind::layout`
names the state layout each kind draws; the catalogue's Media and data view sets
both when switching kind or direction and retains legend/inspection state. Plot
geometry, palette and numeric formatting belong to the consumer. This categorical
contract does not supply time/log scales, per-slice legend selection, zoom,
streaming data ingestion or a separate rendering engine.

## MessageScroller

`MessageScrollerState::new(cx)` owns the existing ScrollHandle plus the previous
frame's stable message keys and reading position. Give
`MessageScroller::new(id, &state, cx)` a bounded height and add each message with
`.item(stable_id, content)`. IDs must be unique within a conversation and also
scope each message's child controls, preserving their identity across reordering.
Create a new state when changing to another conversation. Existing ScrollArea performs
wheel/trackpad scrolling, clipping and scrollbar interaction.

The first frame starts at the bottom. Appending messages, streaming edits and
resize keep following while the reader is at the bottom. When the reader moves
away, the first visible message keeps its pixel position as history is prepended
or earlier items change height; correction happens in the same layout frame.
Removing the anchor uses ordinary native offset clamping. Appends while
away increase `unread_count`, and `jump_to_latest` resumes following without
moving focus from the caller's control. Returning to the bottom clears unread,
including when a resize or content removal reveals the bottom in the same frame.
There is no virtualization or chat transport.

## Verification boundary

Focused tests cover column alignment, table metadata, interactive cell children,
native scrolling and reveal, divider dragging/keyboard behavior, and the
DatePicker-to-resizable-plan example flow. Existing
CI also builds the native example on Linux, Windows, and macOS. Automated GPUI
event/layout checks do not establish native visual, screen-reader, mixed-DPI, or
performance results.
