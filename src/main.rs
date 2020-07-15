mod ui;

use ui::*;

fn main() {
    #[cfg(windows)]
    {
        window_test();
    }

    message_dialog("Great title", "Hello world");
    yes_no_dialog(
        "Do you like boxes?",
        "y/n?",
        || println!("yes pressed"),
        || println!("no pressed"),
    );
}
