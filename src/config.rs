use rand::seq::{IndexedRandom, SliceRandom};
use rand::Rng;
use serde::{Serialize, Deserialize};

use crate::FieldID;

#[derive(Clone, Debug)]
pub struct StartPositions {
    pub mister_x : FieldID,
    pub detectives : Vec<FieldID>
}

/// Configuration & Starting conditions for a game
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameConfig {
    // General
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
        Self {
            reveal_turns: vec![ 3, 8, 13, 18, 24 ],

            mx_start_pos_list: vec![ 
                13, 26, 29, 34, 50, 53, 91, 94, 103, 112, 117, 132, 138, 141, 155, 174, 197, 198 
            ],
            mx_double_turns: 2,
            mx_black_tickets: 5,

            det_start_pos_list: vec![ 
                13, 26, 29, 34, 50, 53, 91, 94, 103, 112, 117, 132, 138, 141, 155, 174, 197, 198 
            ],
            det_taxi_tickets: 11, 
            det_bus_tickets: 7,
            det_metro_tickets: 4,
        }
    }

    /// The maximum turns the game can have
    pub fn max_turns(&self) -> u16 {
        self.det_taxi_tickets + self.det_bus_tickets + self.det_metro_tickets + self.mx_double_turns
    }

    /// Generates starting positions for Mister X and the detectives
    /// 
    /// # Error
    /// 
    /// Returns an error if no starting positions for Mister X are in the set,
    /// or if the number of detectives exceeds the number of starting positions for them
    pub fn gen_start_positions(&self, num_det : usize) -> Result<StartPositions, String> {
        let mister_x = *self.mx_start_pos_list.choose(&mut rand::rng())
            .ok_or(String::from("No valid start positions for Mister X are in this set"))?;

        if num_det > self.det_start_pos_list.len() {
            // If the number of detectives is larger than the number of positions, return an error
            Err("The number of start positions in the set is too small".into())

        } else if num_det == self.det_start_pos_list.len() {
            // If the number of detectives equals the number of start positions, shuffle the array
            let mut detectives = self.det_start_pos_list.clone();
            detectives.shuffle(&mut rand::rng());

            Ok(StartPositions {
                mister_x,
                detectives
            })
    
        } else {
            // If the number of detectives is smaller than the starting positions, create
            let mut vaild_det_pos_list = self.det_start_pos_list.clone();
            let mut detectives = Vec::new();

            // Check if the position of Mister X is in the set of detective positions and remove it if that's the case
            for i in 0 .. vaild_det_pos_list.len() {
                if vaild_det_pos_list[i] == mister_x {
                    vaild_det_pos_list.remove(i);
                    break;
                }
            }

            // Choose positions each and remove them from the valid ones
            for _ in 0 .. num_det {
                let index = rand::rng().random_range(0 .. vaild_det_pos_list.len());
                detectives.push(vaild_det_pos_list[index]);
                vaild_det_pos_list.remove(index);
            }

            Ok(StartPositions {
                mister_x,
                detectives
            })
        }
    }
}

impl Default for GameConfig {
    fn default() -> Self {
        Self::create_classic()
    }
}