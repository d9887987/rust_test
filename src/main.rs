mod user_info;
mod book_info;
mod chapter_two;
mod chapter_there;

use crate::book_info::info_mation;
use crate::book_info::service_info;
use crate::user_info::user;
use crate::user_info::service_go;
use crate::chapter_two::guess_games;
use crate::chapter_there::basic_grammar;


fn main() {
    //guess_games::guess_games_num();
    basic_grammar::variable();
    basic_grammar::loop_control_flow();
    basic_grammar::function(1,3);
    basic_grammar::for_control_flow();
    basic_grammar::if_control_flow(25);
    basic_grammar::while_control_flow(5);

}