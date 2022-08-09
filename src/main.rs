use inputbot::KeybdKey::{
    self, AKey, BKey, CKey, DKey, EKey, FKey, GKey, HKey, IKey, JKey, KKey, LKey, MKey, NKey, OKey,
    PKey, QKey, RKey, SKey, TKey, UKey, VKey, WKey, XKey, YKey, ZKey,
};

const KEYS: [inputbot::KeybdKey; 26] = [
    AKey, BKey, CKey, DKey, EKey, FKey, GKey, HKey, IKey, JKey, KKey, LKey, MKey, NKey, OKey, PKey,
    QKey, RKey, SKey, TKey, UKey, VKey, WKey, XKey, YKey, ZKey,
];

fn key_to_string(key: &KeybdKey) -> char {
    let key_name = format!("{key:?}");

    key_name.chars().next().expect("invalid key name")
}

fn main() {
    // Bind your caps lock key to a function that starts an autoclicker.
    for key in KEYS {
        key.bind(move || {
            println!("key: {}", key_to_string(&key));
        })
    }

    // Call this to start listening for bound inputs.
    inputbot::handle_input_events();
}
