/*
 * ui/input.rs
 *
 * markov-music - A music player that uses Markov chains to choose songs
 * Copyright (c) 2017 Ammon Smith
 *
 * markov-music is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 2 of the License, or
 * (at your option) any later version.
 *
 * markov-music is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with markov-music.  If not, see <http://www.gnu.org/licenses/>.
 */

use pancurses::Input::*;
use pancurses::getmouse;
use ui::Ui;

pub enum Command {
    TogglePause,
    Stop,
    MoveUp,
    MoveDown,
    ParentDir,
    PlaySelected,
    LowerVolume,
    RaiseVolume,
    Mute,
    SeekBack,
    SeekForward,
    SeekStart,
    SeekEnd,
    AddSelected,
    Shuffle,
    Next,
    Previous,
    Repeat,
    LoopBack,
    Like,
    Dislike,
    Random,
    Tired,
    Redraw,
    Quit,
    Abort,
    Nothing,
}

const CTRL_C: char = '\x03';
const CTRL_L: char = '\x0c';

pub fn next_command(ui: &Ui) -> Command {
    if let Some(key) = ui.get_window().getch() {
        match key {
            /* Player control */
            Character(' ') => Command::TogglePause,
            Character('S') => Command::Stop,
            Character('j') | KeyDown => Command::MoveDown,
            Character('k') | KeyUp => Command::MoveUp,
            Character('h') | KeyLeft => Command::ParentDir,
            Character('\n') | Character('l') | KeyRight => Command::PlaySelected,
            Character('<') => Command::LowerVolume,
            Character('>') => Command::RaiseVolume,
            Character('m') => Command::Mute,
            Character('[') => Command::SeekBack,
            Character(']') => Command::SeekForward,
            Character('{') => Command::SeekStart,
            Character('}') => Command::SeekEnd,

            /* Markov control */
            Character('a') => Command::AddSelected,
            Character('s') | KeyRedo => Command::Shuffle,
            Character('n') | KeyNext => Command::Next,
            Character('p') | KeyPrevious => Command::Previous,
            Character('r') => Command::Repeat,
            Character('o') => Command::LoopBack,
            Character('+') => Command::Like,
            Character('-') => Command::Dislike,
            Character('R') => Command::Random,
            Character('t') => Command::Tired,

            /* Misc */
            KeyMouse => process_mouse(),
            Character(CTRL_L) | KeyClear | KeyResize => Command::Redraw,
            Character('q') | KeyClose | KeyExit => Command::Quit,
            Character(CTRL_C) | KeyCancel => Command::Abort,
            _ => Command::Nothing,
        }
    } else {
        Command::Nothing
    }
}

fn process_mouse() -> Command {
    let mevt = getmouse().expect("Not mouse event despite character event");
    let _ = mevt;
    Command::Nothing
}
