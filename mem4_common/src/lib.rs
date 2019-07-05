//! `mem4_common` - commons for mem4 wasm and server
//! Learning to code Rust for a http + WebSocket  

//region: Clippy
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    //variable shadowing is idiomatic to Rust, but unnatural to me.
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::shadow_unrelated,
)]
#![allow(
    //library from dependencies have this clippy warnings. Not my code.
    clippy::cargo_common_metadata,
    clippy::multiple_crate_versions,
    clippy::wildcard_dependencies,
    //Rust is more idiomatic without return statement
    clippy::implicit_return,
    //I have private function inside a function. Self does not work there.
    //clippy::use_self,
    //Cannot add #[inline] to the start function with #[wasm_bindgen(start)]
    //because then wasm-pack build --target no-modules returns an error: export `run` not found 
    clippy::missing_inline_in_public_items,
    //Why is this bad : Doc is good. rustc has a MISSING_DOCS allowed-by-default lint for public members, but has no way to enforce documentation of private items. This lint fixes that.
    clippy::doc_markdown,
)]
//endregion

//region: extern and use statements
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate strum_macros;

use strum_macros::AsRefStr;
//endregion

///`WsMessage` enum for WebSocket
#[derive(Serialize, Deserialize)]
pub enum WsMessage {
    ///Dummy
    Dummy {
        ///anything
        dummy: String,
    },
    ///Request WebSocket Uid - first message to WebSocket server
    RequestWsUid {
        ///anything
        test: String,
    },
    ///response from WebSocket server for first message
    ResponseWsUid {
        ///WebSocket Uid
        your_ws_uid: usize,
    },
    ///want to play
    WantToPlay {
        ///ws client instance unique id. To not listen the echo to yourself.
        my_ws_uid: usize,
        ///content folder name
        asked_folder_name: String,
    },
    /// accept play
    PlayAccept {
        ///ws client instance unique id. To not listen the echo to yourself.
        my_ws_uid: usize,
        ///json of vector of players
        players: String,
    },
    /// player1 initialize the game data and sends it to all players
    /// I will send json string to not confuse the server with vectors
    GameDataInit {
        ///act is the action to take on the receiver
        card_grid_data: String,
        ///json of game_config
        game_config: String,
        ///json of vector of players
        players: String,
    },
    ///player click
    PlayerClick1Card {
        ///this identifies the smartphone, but not the player-in-turn
        my_ws_uid: usize,
        ///all players
        players: String,
        ///card index
        card_index: usize,
        ///game status PlayerBefore1Card or PlayerBefore2Card
        game_status: GameStatus,
    },
    ///player click
    PlayerClick2Card {
        ///this identifies the smartphone, but not the player-in-turn
        my_ws_uid: usize,
        ///all players
        players: String,
        ///card index
        card_index: usize,
        ///game status PlayerBefore1Card or PlayerBefore2Card
        game_status: GameStatus,
    },
    ///Play Again
    PlayAgain {
        ///this identifies the smartphone, but not the player-in-turn
        my_ws_uid: usize,
        ///all players
        players: String,
    },
    ///player change
    TakeTurnEnd {
        ///ws client instance unique id. To not listen the echo to yourself.
        my_ws_uid: usize,
        ///all players
        players: String,
    },
    ///Request the game_config from the WebSocket server
    RequestGameConfig {
        ///the file with the game_config
        filename: String,
    },
    ///Receive the game_config from the WebSocket server
    ResponseGameConfigJson {
        ///the game_config from the server
        json: String,
    },
}

///the game can be in various statuses and that differentiate the UI and actions
/// all players have the same game status
#[derive(AsRefStr, Serialize, Deserialize, Clone)]
pub enum GameStatus {
    /// want to play ask begin
    WantToPlayAskBegin,
    ///Player1 WantToPlay Asking
    WantToPlayAsking,
    ///Player2 WantToPlay Asked
    WantToPlayAsked,
    ///PlayAccepted
    PlayAccepted,
    ///Play before first card
    PlayBefore1Card,
    ///Play before second card
    PlayBefore2Card,
    ///take turn begin
    TakeTurnBegin,
    ///take turn end
    TakeTurnEnd,
    ///end game
    PlayAgain,
    ///Reconnect after a lost connection
    Reconnect,
}

///data for one player
#[derive(Serialize, Deserialize)]
pub struct Player {
    ///ws_uid
    pub ws_uid: usize,
    ///field for src attribute for HTML element image and filename of card image
    pub points: usize,
}
//endregion
