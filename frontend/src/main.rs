use slint::ComponentHandle;

fn main() -> Result<(), slint::PlatformError> {
    let ui = frontend::setup_ui()?;
    ui.run()
}
