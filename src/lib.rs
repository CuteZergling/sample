extern crate num;
mod command;

use command::{command,/* delay, sleep_ms,*/ COMMANDS};
extern crate num_derive;

#[no_mangle]
pub unsafe extern "C" fn main() -> Vec<String> {
   use rand::Rng;
   use rand::prelude::*;
    
    COMMANDS.clear();
    
    let mut rng = rand::thread_rng();
    let store = rng.gen_range(1..65);

        let mut rng = rand::thread_rng();
        let arr: [u8; 30] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30,];
        let sample: Vec<_> = arr.choose_multiple (&mut rng, 4).collect();
        
    
    
    
    command!("say {},{},{},{}",sample[0], sample[1], sample[2], sample[3]);

    return COMMANDS.clone();
}
