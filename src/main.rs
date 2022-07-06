#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;
mod row;
mod document;

use editor::Editor;

fn main() {
    Editor::new().run();
}

