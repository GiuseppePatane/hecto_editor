mod editor;
mod terminal;

use crate::editor::Editor;

fn main() {
    println!("press ctrl +q to quit");
    Editor::default().run();
}
