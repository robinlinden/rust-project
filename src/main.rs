mod ui;

use ui::*;

fn main() {
    message_dialog("Great title", "Hello world");
    yes_no_dialog(
        "Do you like boxes?",
        "y/n?",
        || println!("yes pressed"),
        || println!("no pressed"),
    );
}
