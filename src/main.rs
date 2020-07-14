mod ui;

use ui::*;

fn main() {
    #[cfg(windows)]
    {
        register_a_class();
    }

    message_dialog("Great title", "Hello world");
    yes_no_dialog(
        "Do you like boxes?",
        "y/n?",
        || println!("yes pressed"),
        || println!("no pressed"),
    );
}
