//! Source code example of how to create your own widget.
//! This is meant to be read as a tutorial, hence the plethora of comments.
use crate::*;

/// iOS-style toggle switch:
///
/// ``` text
///      _____________
///     /       /.....\
///    |       |.......|
///     \_______\_____/
/// ```
pub fn toggle(ui: &mut Ui, on: &mut bool) -> Response {
    // Widget code can be broken up in four steps:
    //  1. Decide a size for the widget
    //  2. Request space for it
    //  3. Handle interactions with the widget (if any)
    //  4. Paint the widget

    // 1. Deciding widget size:
    // You can query the `ui` how much space is available,
    // but in this example we have a fixed size widget of the default size for a button:
    let desired_size = ui.style().spacing.interact_size;

    // 2. Requesting space:
    // This is where we get a region (`Rect`) of the screen assigned.
    let rect = ui.request_space(desired_size);

    // If we get `None` back from `request_space`, it means this widgets isn't visible.
    // In this case we shouldn't do anything else and just return early.
    // Egui has a helper macro for this:
    let rect = unwrap_or_return_default!(rect);

    // 3. Interact: Time to check for clicks!
    // To do that we need an `Id` for the button.
    // Id's are best created from unique identifiers (like fixed labels)
    // but since we have no label for the switch we here just generate an `Id` automatically
    // (based on a rolling counter in the `Ui`).
    let id = ui.make_position_id();
    let response = ui.interact(rect, id, Sense::click());
    if response.clicked {
        *on = !*on;
    }

    // 4. Paint!
    // First let's ask for a simple animation from Egui.
    // Egui keeps track of changes in the boolean associated with the id and
    // returns an animated value in the 0-1 range for how much "on" we are.
    let how_on = ui.ctx().animate_bool(id, *on);
    // We will follow the current style by asking
    // "how should something that is being interacted with be painted?".
    // This will, for instance, give us different colors when the widget is hovered or clicked.
    let visuals = ui.style().interact(&response);
    let off_bg_fill = Rgba::new(0.0, 0.0, 0.0, 0.0);
    let on_bg_fill = Rgba::new(0.0, 0.5, 0.25, 1.0);
    let bg_fill = lerp(off_bg_fill..=on_bg_fill, how_on);
    // All coordinates are in absolute screen coordinates so we use `rect` to place the elements.
    let radius = 0.5 * rect.height();
    ui.painter().rect(rect, radius, bg_fill, visuals.bg_stroke);
    // Paint the circle, animating it from left to right with `how_on`:
    let circle_x = lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
    let center = pos2(circle_x, rect.center().y);
    ui.painter()
        .circle(center, 0.75 * radius, visuals.fg_fill, visuals.fg_stroke);

    // All done! Return the interaction response so the user can check what happened
    // (hovered, clicked, ...) and maybe show a tooltip:
    response
}

pub fn demo(ui: &mut Ui, on: &mut bool) {
    ui.label("It's easy to create your own widgets!");
    let url = format!("https://github.com/emilk/egui/blob/master/{}", file!());
    ui.horizontal(|ui| {
        ui.label("Like this toggle switch:");
        toggle(ui, on).on_hover_text("Click to toggle");
        ui.add(Hyperlink::new(url).text("(source code)"));
    });
}
