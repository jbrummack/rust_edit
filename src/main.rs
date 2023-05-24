use bevy::
{
    prelude::*, 
    window::{ReceivedCharacter, FileDragAndDrop}, 
    //diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin}, 
    input::keyboard::KeyboardInput, 
    winit::{
        WinitSettings,
    }
};

use std::env;
use std::fs;


///--------TODO----------
//Create text section system https://docs.rs/bevy/latest/bevy/text/struct.TextSection.html
//  Build a Buffer that holds the style information for the sections (propably by their indexes)
//Create Syntax highlighting which interfaces with the text section system use regex or JS
//File reading
//UI & Cursor

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .init_resource::<RenderedText>()
        .add_startup_system(setup)
        .add_system(text_in_system)
        .add_system(text_out_system)
        .run();
}

#[derive(Component)]
struct WritableText;

#[derive(Clone, Resource)]
struct RenderedText {
    current_line: usize,
    lines: usize,
    current_char: usize,
    contents: String,
}
#[derive(Clone, Resource)]
struct UiResource {
    cursor_line: usize,
    lines: usize,
    cursor_char: usize,
    contents: String,
    numbers: String,
}
#[derive(Clone)]
struct TextButton {
    label: String,
    clicked: bool,
}
#[derive(Clone)]
struct MenuBar {
    label: String,
    clicked: bool,
}

impl FromWorld for RenderedText {
    fn from_world(world: &mut World) -> Self {
        RenderedText::new()
    }
}


fn text_out_system(mut query: Query<&mut Text, With<WritableText>>, text_buffer: Res<RenderedText>){


    //Instead of outputting the buffer directly: plug the string through syntax higligher and then into section generator
    for mut text in &mut query {
            text.sections[1].value = format!("{}", text_buffer.contents);
            info!("Text Section Amount {}", text.sections.len());
}
}

fn text_in_system(mut char_input_events: EventReader<ReceivedCharacter>, sonder: Res<Input<KeyCode>>, mut text_buffer: ResMut<RenderedText>) {
    let mut inhibit_char = false;

    if sonder.just_pressed(KeyCode::Up) {
        text_buffer.move_line(-1);
        inhibit_char = true;
    }
    if sonder.just_pressed(KeyCode::Down) {
        text_buffer.move_line(1);
        inhibit_char = true;
    }
    
    if sonder.just_pressed(KeyCode::Right) {
        text_buffer.move_cursor(1);
        inhibit_char = true;
    }
    if sonder.just_pressed(KeyCode::Left) {
        text_buffer.move_cursor(-1);

        inhibit_char = true;
    }
    if sonder.just_pressed(KeyCode::Back) {
        text_buffer.backspace();
        inhibit_char = true;
    }

    //let current_line = text_buffer.current_line;
    if !inhibit_char {
        for event in char_input_events.iter() {
            info!("{:?}: '{}' {:?} {}", event, event.char, sonder, text_buffer.current_char);
            text_buffer.add_char(event.char);
            info!("'{}'", text_buffer.contents);
        }
    }
    
}



fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {


    // UI camera
    commands.spawn(Camera2dBundle::default());
    // Text with multiple sections
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "1",
                TextStyle {
                    font: asset_server.load("fonts/Vera/VeraMoBd.ttf"),
                    font_size: 30.0,
                    color: Color::GOLD,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/Vera/VeraMono.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            }),
        ]),
        WritableText,
    ));
}


fn file_drop(mut dnd_evr: EventReader<FileDragAndDrop>,) {
    for ev in dnd_evr.iter() {
        println!("{:?}", ev);
        if let FileDragAndDrop::DroppedFile { window, path_buf } = ev {
            println!("Dropped file with path: {:?}, in window id: {:?}", path_buf, window);
        }
    }
}

fn read_file(path: String) -> String{
    // --snip--
    println!("In file {}", path);

    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");

    contents
}

impl RenderedText {

    fn new() -> Self{
        Self{
            current_line: 0,
            lines: 0,
            current_char: 0,
            contents: "HelloWorld".to_string(),
        }
    }
    fn from_file(file_path: String) -> Self{
        Self{
            current_line: 0,
            lines: 0,
            current_char: 0,
            contents: read_file(file_path),
        }
    }

    fn add_char(&mut self, input: char){
        let mut modified_text = self.clone();
        modified_text.contents.insert(self.current_char, input);
        modified_text.current_char += 1;

        *self = modified_text;
    }

    fn backspace(&mut self){
        let mut modified_text = self.clone();
        if modified_text.current_char > 0 {
            modified_text.current_char -= 1;
            modified_text.contents.remove(self.current_char - 1);
        }

        *self = modified_text;
    }
    fn move_cursor(&mut self, offset: i32){
        let mut bound_check_triggered = false;
        if(self.current_char < 1 && offset < 1){
            bound_check_triggered = true;
        }
        if(self.current_char as i32 + offset) > self.contents.len() as i32{
            bound_check_triggered = true;
        }
        if !bound_check_triggered {
            let mut modified_text = self.clone();
            modified_text.current_char = modified_text.current_char + offset as usize;
            *self = modified_text;
        }
    }
    fn move_line(&mut self, offset: i32){
        self.recount_lines();
        let mut bound_check_triggered = false;
        if(self.current_line < 1 && offset < 1){
            bound_check_triggered = true;
        }
        if(self.current_line as i32 + offset) > self.lines as i32{
            bound_check_triggered = true;
        }
        if !bound_check_triggered {
            let mut modified_text = self.clone();
            modified_text.current_line = modified_text.current_line + offset as usize;
            *self = modified_text;
        }
    }

    fn recount_lines(&mut self){
        let mut modified_text = self.clone();
        modified_text.lines = modified_text.contents.matches("\r").count();

        *self = modified_text;
    }
}

