use modhost::{Tag, tags};

pub fn tags() -> Vec<Tag> {
    tags![
        "Cosmetic", "Cosmetic", "tabler:palette";
        "Developer", "Developer", "tabler:code";
        "Gameplay", "Gameplay", "tabler:device-gamepad-2";
        "Joke", "Joke", "tabler:mood-wink";
        "Other", "Other", "tabler:jetpack";
        "Tweaks", "Tweaks", "tabler:edit";
    ]
}
