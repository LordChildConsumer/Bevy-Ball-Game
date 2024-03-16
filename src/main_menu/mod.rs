/*
    #############################
    ##                         ##
    ##        MAIN MENU        ##
    ##                         ##
    #############################
*/


use bevy::prelude::*;



pub struct MainManuPlugin;

impl Plugin for MainManuPlugin {
    fn build(&self, app: &mut App) {
        
        // Startup
        app.add_systems(Startup, main_menu);
    
    }
}




/*
    ----------------------------
    ---- Placeholder System ----
    ----------------------------
*/

pub fn main_menu() {
    println!("Main Menu!");
}