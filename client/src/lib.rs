use bevy::prelude::{App, Input, KeyCode, Plugin, Res};

impl Plugin for PotooClient {
    // this is where we set up our plugin
    fn build(&self, app: &mut App) {
        app.add_system(contact_http);
    }
}
pub struct PotooClient;

fn contact_http(keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::H) {
        println!("Hello World");
        //Send request to server
    }
}
