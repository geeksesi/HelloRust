use hotkey;

// look this https://github.com/unwrittenfun/hotkey-rs
fn main() {
    let mut hk = hotkey::Listener::new();
    hk.register_hotkey(
        hotkey::modifiers::CONTROL | hotkey::modifiers::SHIFT,
        'A' as u32,
        || println!("Ctrl-Shift-A pressed!"),
    )
    .unwrap();

    hk.listen();
}
