use native_dialog::MessageDialog;

fn main() {
    MessageDialog::new()
        .set_title("Hello")
        .set_text("Hello, world!")
        .show_alert()
        .unwrap();
}
