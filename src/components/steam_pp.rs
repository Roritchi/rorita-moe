use leptos::prelude::*;

/// A parameterized incrementing button
#[component]
pub fn SteamProfilePicture() -> impl IntoView {
    view! {
        <div class="playerAvatar profile_header_size offline" data-miniprofile="164815833">
            <div class="playerAvatarAutoSizeInner">
                <div class="profile_avatar_frame">
                    <img src="https://cdn.fastly.steamstatic.com/steamcommunity/public/images/items/1069740/85030942387d8c7803922f84c31e82bc42728279.png" />
                </div>
                <img src="https://avatars.fastly.steamstatic.com/fc433d7ca29e87777030d24657ffbb94b98a5cca_full.jpg" />
            </div>
        </div>
    }
}
