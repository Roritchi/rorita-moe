use leptos::prelude::*;

/// A parameterized incrementing button
#[component]
pub fn SteamProfilePicture() -> impl IntoView {
    view! {
        <div class="steam-pp-wrapper">
            <div class="playerAvatar profile_header_size">
                <div class="playerAvatarAutoSizeInner">
                    <div class="profile_avatar_frame">
                        <img src="/images/steam-pp-border.png" />
                    </div>
                    <img src="/images/steam-pp.jpg" />
                </div>
            </div>
        </div>
    }
}
