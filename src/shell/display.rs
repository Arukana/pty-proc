use std::collections::VecDeque;
use std::{io, fmt};

use ::termion;
use ::termion::cursor::{Goto};
use ::libc;

#[derive(Clone)]
pub struct Display {
  screen: VecDeque<u8>,
  col: libc::c_ushort,
  row: libc::c_ushort,
}

impl Display
{ pub fn to_matrix(&self) -> Vec<Vec<u8>>
  { let mut matrix: Vec<Vec<u8>> = Vec::with_capacity(self.row as usize);
    let mut flag: bool = true;
    let mut tmp = self.screen.iter();
    {0..self.row}.all(|y|
    { let mut coucou: Vec<u8> = Vec::with_capacity(self.col as usize);
      {0..self.col}.all(|x|
      { if flag
        { if let Some(k) = tmp.next()
          { coucou.push(*k);
          if *k == 10
          { flag = false; }}}
        else
        { coucou.push(0); }
        true });
      matrix.push(coucou);
      true });
    matrix }}

impl ExactSizeIterator for Display {
  fn len(&self) -> usize {
    self.col.checked_mul(self.row).unwrap_or_default() as usize
  }
}

impl Iterator for Display {
  type Item = u8;

  fn next(&mut self) -> Option<u8> {
    match self.screen.iter().next() {
      Some(&term) => Some(term),
      None => None,
    }
  }
}

impl fmt::Display for Display {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}{}",
      Goto(1, 1),
      String::from_utf8_lossy(self.screen.as_slices().0),
    )
  }
}

impl io::Write for Display {

  /// The method `write` from trait `io::Write` inserts a new list of terms
  /// from output.
  fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    Ok(buf.iter().map(|&term| {
      self.screen.push_back(term);
    }).count())
  }

  fn flush(&mut self) -> io::Result<()> {
    Ok(())
  }
}

impl io::Read for Display {

  /// The method `read` from trait `io::Read` sets the screen to
  /// the argument buffer.
  fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    Ok(buf.iter_mut().zip(self.screen.iter())
                     .map(|(mut dest, src)| {
      *dest = *src;
    }).count())
  }
}

impl Default for Display {

  /// The constructor method `default` returns the `Display`'s interface
  /// from shell.
  fn default() -> Display {
    let (col, row): (libc::c_ushort, libc::c_ushort) = termion::terminal_size()
                                                               .unwrap_or((
      ::DISPLAY_DEFAULT_COL,
      ::DISPLAY_DEFAULT_ROW
    ));

    Display {
      screen: VecDeque::with_capacity(col.checked_mul(row).unwrap_or_default() as usize),
      col: ::DISPLAY_DEFAULT_COL,
      row: ::DISPLAY_DEFAULT_ROW,
    }
  }
}
