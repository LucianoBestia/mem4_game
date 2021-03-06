//! statusinviteasked.rs - code flow from this status

//region: use
use crate::rootrenderingcomponent::RootRenderingComponent;
use crate::websocketcommunication;
use crate::logmod;

use dodrio::builder::text;
use dodrio::bumpalo::{self, Bump};
use dodrio::Node;
use mem4_common::{GameStatus, Player, WsMessage};
use typed_html::dodrio;
//endregion

///render asked
pub fn div_invite_asked<'a, 'bump>(
    root_rendering_component: &'a RootRenderingComponent,
    bump: &'bump Bump,
) -> Node<'bump>
where
    'a: 'bump,
{
    // 2S Click here to Accept play!
    logmod::log1_str("GameStatus::InviteAsked");
    //return Click here to Accept play
    dodrio!(bump,
    <div class="div_clickable" onclick={move |root, vdom, _event| {
                let rrc = root.unwrap_mut::<RootRenderingComponent>();
                div_invite_asked_on_click(rrc);
                vdom.schedule_render();
            }}>
        <h2 id= "ws_elem" style= "color:green;">
                {vec![text(
                    //show Ask Player2 to Play!
                    bumpalo::format!(in bump, "Click here to Accept {}!", root_rendering_component.game_data.asked_folder_name)
                        .into_bump_str(),
                )]}
        </h2>
    </div>
    )
}

/// on click
pub fn div_invite_asked_on_click(rrc: &mut RootRenderingComponent) {
    rrc.game_data.game_status = GameStatus::PlayAccepted;

    websocketcommunication::ws_send_msg(
        &rrc.game_data.ws,
        &WsMessage::PlayAccept {
            my_ws_uid: rrc.game_data.my_ws_uid,
            players: unwrap!(serde_json::to_string(&rrc.game_data.players)),
        },
    );
}

///msg accept play
pub fn on_msg_play_accept(rrc: &mut RootRenderingComponent, my_ws_uid: usize) {
    if rrc.game_data.my_player_number == 1 {
        rrc.game_data.players.push(Player {
            ws_uid: my_ws_uid,
            points: 0,
        });
        rrc.check_invalidate_for_all_components();
    }
}

///render play accepted
pub fn div_play_accepted<'a, 'bump>(
    rrc: &'a RootRenderingComponent,
    bump: &'bump Bump,
) -> Node<'bump>
where
    'a: 'bump,
{
    logmod::log1_str("GameStatus::PlayAccepted");
    dodrio!(bump,
    <h2 id= "ws_elem" style= "color:red;">
        {vec![text(bumpalo::format!(in bump, "Game {} accepted.", rrc.game_data.asked_folder_name).into_bump_str(),)]}
    </h2>
    )
}
