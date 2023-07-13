extern crate num;
mod command;

use command::{command,/* delay, sleep_ms,*/ COMMANDS};
extern crate num_derive;

#[no_mangle]
pub unsafe extern "C" fn main() -> Vec<String> {
   use rand::Rng;
    
    COMMANDS.clear();
    
    let mut rng = rand::thread_rng();
    let store = rng.gen_range(1..65);

    command!("scoreboard players set random64to1 random64to1 {}",store);

    return COMMANDS.clone();
}
