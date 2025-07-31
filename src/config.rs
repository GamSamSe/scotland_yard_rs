use serde::{Serialize, Deserialize};

use crate::FieldID;

/// Configuration & Starting conditions for a game
#[derive(Serialize, Deserialize)]
pub struct GameConfig {
    // General
        /// Maximum turns of a game until Mister X has won
        pub max_turns : u16,
        /// Turns at which Mister X will be revealed
        pub reveal_turns : Vec<u16>,
    // 

    // Mister X
        /// Possible starting positions of MisterX
        pub mx_start_pos_list : Vec<FieldID>,
        /// Number of double turns Mister X has
        pub mx_double_turns : u16,
        /// Number of black tickets Mister X has
        pub mx_black_tickets : u16,
    // 

    // Detectives 
        /// Possible starting positions for the detectives
        pub det_start_pos_list : Vec<FieldID>,
        /// Number of taxi tickets each detective has
        pub det_taxi_tickets : u16,
        /// Number of bus tickets each detective has
        pub det_bus_tickets : u16,
        /// Number of metro tickets each detective has
        pub det_metro_tickets : u16
    // 
}


impl GameConfig {
    /// Create a new game with the classic rule set
    pub fn create_classic() -> Self {
        // TODO: Create valid start positions

        Self {
            max_turns: 24,
            reveal_turns: vec![ 3, 8, 13, 18, 24 ],

            mx_start_pos_list: vec![ 13, 26, 29, 34, 50, 53, 91, 94, 103, 112, 117, 132, 138, 141, 155, 174, 197, 198 ],
            mx_double_turns: 2,
            mx_black_tickets: 5,

            det_start_pos_list: vec![ 13, 26, 29, 34, 50, 53, 91, 94, 103, 112, 117, 132, 138, 141, 155, 174, 197, 198 ],
            det_taxi_tickets: 11, 
            det_bus_tickets: 7,
            det_metro_tickets: 4,
        }
    }
}