//! game data - structs and methods around game data

//region: extern, use,
extern crate mem4_common;
use crate::logmod;

use mem4_common::{GameStatus, Player};
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::FromEntropy;
use rand::Rng;
use strum_macros::AsRefStr;
use web_sys::WebSocket;
//endregion

//region: struct, enum
///2d size (any UM -pixel, items, percent)
#[derive(Serialize, Deserialize, Clone)]
pub struct Size2d {
    ///horizontal
    pub hor: usize,
    ///vertical
    pub ver: usize,
}
///game config
#[derive(Serialize, Deserialize, Clone)]
pub struct GameConfig {
    ///card moniker - the text/name of the card
    ///the zero element is card face down or empty, example alphabet begins with index 01 : A
    pub card_moniker: Vec<String>,
    ///img filenames
    pub img_filename: Vec<String>,
    ///sound filenames
    pub sound_filename: Vec<String>,
    ///card image width
    pub card_width: usize,
    ///card image height
    pub card_height: usize,
    ///number of cards horizontally
    pub grid_items_hor: usize,
    ///number of card vertically
    pub grid_items_ver: usize,
}

///the 3 possible statuss of one card
#[derive(Serialize, Deserialize, AsRefStr)]
pub enum CardStatusCardFace {
    ///card face down
    Down,
    ///card face Up Temporary
    UpTemporary,
    ///card face up Permanently
    UpPermanently,
}
///all the data for one card
#[derive(Serialize, Deserialize)]
pub struct Card {
    ///card status
    pub status: CardStatusCardFace,
    ///field for src attribute for HTML element imagea and filename of card image
    pub card_number_and_img_src: usize,
    ///field for id attribute for HTML element image contains the card index
    pub card_index_and_id: usize,
}
///game data
pub struct GameData {
    ///game status: InviteAskBegin,InviteAsking,InviteAsked,Player1,Player2
    pub game_status: GameStatus,
    ///vector of cards
    pub card_grid_data: Vec<Card>,
    ///card index of first click
    pub card_index_of_first_click: usize,
    ///card index of second click
    pub card_index_of_second_click: usize,
    ///web socket. used it to send message onclick.
    pub ws: WebSocket,
    ///my ws client instance unique id. To not listen the echo to yourself.
    pub my_ws_uid: usize,
    ///players
    pub players: Vec<Player>,
    ///content folder name
    pub content_folder_name: String,
    ///invite asks for a specific game
    pub asked_folder_name: String,
    ///What player am I
    pub my_player_number: usize,
    ///whose turn is now:  player 1,2,3,...
    pub player_turn: usize,
    ///content folders vector
    pub content_folders: Vec<String>,
    ///game_configs
    pub game_config: Option<GameConfig>,
    ///error text
    pub error_text: String,
    ///href
    pub href: String,
    /// is reconnect
    pub is_reconnect: bool,
}
//endregion

impl GameData {
    ///prepare new random data
    pub fn prepare_random_data(&mut self) {
        let item_count_minus_one = unwrap!(unwrap!(self.game_config.as_ref())
            .card_moniker
            .len()
            .checked_sub(1));
        let players_count = self.players.len();
        let cards_count = unwrap!(players_count.checked_mul(unwrap!(unwrap!(self
            .game_config
            .as_ref())
        .grid_items_hor
        .checked_mul(unwrap!(self.game_config.as_ref()).grid_items_ver))));
        let random_count = unwrap!(cards_count.checked_div(2));
        //if the number of cards is bigger than the images, i choose all the images.
        //for the rest I use random.
        //integer division rounds toward zero
        let multiple: usize = unwrap!(random_count.checked_div(item_count_minus_one));
        let rest =
            unwrap!(random_count.checked_sub(unwrap!(item_count_minus_one.checked_mul(multiple))));

        logmod::log1_str(&format!(
            "item_count_minus_one {}  players_count {} cards_count {} random_count {} multiple {} rest {}",
            item_count_minus_one,players_count,cards_count,random_count,multiple,
            rest,
        ));

        //region: find random numbers between 1 and item_count
        //vec_of_random_numbers is 0 based
        let mut vec_of_random_numbers = Vec::new();
        let mut rng = SmallRng::from_entropy();
        vec_of_random_numbers.clear();
        for _i in 1..=rest {
            //how to avoid duplicates
            let mut num: usize;
            // a do-while is written as a  loop-break
            loop {
                //gen_range is lower inclusive, upper exclusive 26 + 1
                num = rng.gen_range(1, unwrap!(item_count_minus_one.checked_add(1)));
                if !vec_of_random_numbers.contains(&num) {
                    break;
                }
            }
            //push a pair of the same number
            vec_of_random_numbers.push(num);
            vec_of_random_numbers.push(num);
        }
        for _m in 1..=multiple {
            for i in 1..=item_count_minus_one {
                vec_of_random_numbers.push(i);
                vec_of_random_numbers.push(i);
            }
        }
        //endregion

        //region: shuffle the numbers
        let vrndslice = vec_of_random_numbers.as_mut_slice();
        vrndslice.shuffle(&mut rng);
        //endregion

        //region: create Cards from random numbers
        let mut card_grid_data = Vec::new();

        //Index 0 is special and reserved for FaceDown. Cards start with base 1
        let new_card = Card {
            status: CardStatusCardFace::Down,
            card_number_and_img_src: 0,
            card_index_and_id: 0,
        };
        card_grid_data.push(new_card);

        //create cards and push to the vector
        for (index, random_number) in vec_of_random_numbers.iter().enumerate() {
            let new_card = Card {
                status: CardStatusCardFace::Down,
                //dereference random number from iterator
                card_number_and_img_src: *random_number,
                //card base index will be 1. 0 is reserved for FaceDown.
                card_index_and_id: unwrap!(index.checked_add(1), "usize overflow"),
            };
            card_grid_data.push(new_card);
        }
        //endregion
        self.card_grid_data = card_grid_data;
        logmod::log1_str(&format!(
            "vec_of_random_numbers.len {} card_grid_data.len {}",
            vec_of_random_numbers.len(),
            self.card_grid_data.len()
        ));
    }
    ///asociated function: before Accept, there are not random numbers, just default cards.
    pub fn prepare_for_empty() -> Vec<Card> {
        //prepare 32 empty cards. The random is calculated only on PlayAccept.
        let mut card_grid_data = Vec::new();
        //I must prepare the 0 index, but then I don't use it ever.
        for i in 0..=32 {
            let new_card = Card {
                status: CardStatusCardFace::Down,
                card_number_and_img_src: 1,
                card_index_and_id: i,
            };
            card_grid_data.push(new_card);
        }
        card_grid_data
    }
    ///constructor of game data
    pub fn new(ws: WebSocket, my_ws_uid: usize) -> Self {
        let mut players = Vec::new();
        players.push(Player {
            ws_uid: 0,
            points: 0,
        });
        //return from constructor
        GameData {
            card_grid_data: Self::prepare_for_empty(),
            card_index_of_first_click: 0,
            card_index_of_second_click: 0,
            ws,
            my_ws_uid,
            players,
            game_status: GameStatus::InviteAskBegin,
            content_folder_name: "alphabet".to_string(),
            asked_folder_name: "".to_string(),
            my_player_number: 1,
            player_turn: 0,
            content_folders: vec![
                String::from("alphabet"),
                String::from("animals"),
                String::from("playingcards"),
                String::from("triestine"),
            ],
            game_config: None,
            error_text: "".to_string(),
            href: "".to_string(),
            is_reconnect: false,
        }
    }
    ///check only if status InviteAskBegin
    pub fn is_status_invite_ask_begin(&self) -> bool {
        #[allow(clippy::wildcard_enum_match_arm)]
        match self.game_status {
            GameStatus::InviteAskBegin => true,
            _ => false,
        }
    }
    ///the only statuss for rendering the grid container
    pub fn is_status_for_grid_container(&self) -> bool {
        #[allow(clippy::wildcard_enum_match_arm)]
        match self.game_status {
            GameStatus::PlayBefore1stCard
            | GameStatus::PlayBefore2ndCard
            | GameStatus::TakeTurnBegin
            | GameStatus::TakeTurnEnd
            | GameStatus::GameOverPlayAgainBegin => true,
            _ => false,
        }
    }
}
