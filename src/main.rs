mod user_info;
mod book_info;
mod chapter_two;

use crate::book_info::info_mation;
use crate::book_info::service_info;
use crate::user_info::user;
use crate::user_info::service_go;
use crate::chapter_two::guess_games;


fn main() {
    guess_games::guess_games_num();
}