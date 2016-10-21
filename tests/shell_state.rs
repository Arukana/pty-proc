extern crate pty_proc;
extern crate libc;

use std::thread;
use std::time;

use self::pty_proc::prelude::*;

const A: In = [b'a',
               b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'
];

const B: In = [b'b',
            b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'
];

#[test]
fn test_key_down() {
    let mut state: ShellState = ShellState::new(None, None, libc::STDIN_FILENO);

    state.set_input(Some(Control::new(A, 1)));
    assert_eq!(state.is_input_keydown(), Some(Key::new(&A, 1)));
    state.set_input(Some(Control::new(A, 1)));
    assert_eq!(state.is_input_keydown(), Some(Key::new(&A, 1)));
    state.set_input(Some(Control::new(B, 1)));
    assert_eq!(state.is_input_keydown(), Some(Key::new(&B, 1)));
    state.set_input(None);
    assert_eq!(state.is_input_keydown(), None);
}

#[test]
fn test_key_repeat() {
    let mut state: ShellState = ShellState::new(Some(REPEAT), None, libc::STDIN_FILENO);

    state.set_input(Some(Control::new(A, 1)));
    state.set_input(Some(Control::new(A, 1)));
    state.set_input(None);
    state.set_input(Some(Control::new(A, 1)));
    assert_eq!(state.is_input_keyrepeat(), Some(2));
    thread::sleep(time::Duration::from_millis(REPEAT as u64));
    state.set_input(None);

    assert_eq!(state.is_input_keyrepeat(), Some(0));
    state.set_input(Some(Control::new(A, 1)));
    state.set_input(Some(Control::new(A, 1)));
    assert_eq!(state.is_input_keyrepeat(), Some(1));
    state.set_input(Some(Control::new(B, 1)));
    assert_eq!(state.is_input_keyrepeat(), Some(0));
}
