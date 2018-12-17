// options.rs

// *************************************************************************
// * Copyright (C) 2018 Daniel Mueller (deso@posteo.net)                   *
// *                                                                       *
// * This program is free software: you can redistribute it and/or modify  *
// * it under the terms of the GNU General Public License as published by  *
// * the Free Software Foundation, either version 3 of the License, or     *
// * (at your option) any later version.                                   *
// *                                                                       *
// * This program is distributed in the hope that it will be useful,       *
// * but WITHOUT ANY WARRANTY; without even the implied warranty of        *
// * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the         *
// * GNU General Public License for more details.                          *
// *                                                                       *
// * You should have received a copy of the GNU General Public License     *
// * along with this program.  If not, see <http://www.gnu.org/licenses/>. *
// *************************************************************************

use std::fmt;
use std::io;
use std::str::FromStr;

use crate::commands;
use crate::error::Error;
use crate::Result;

/// A top-level command for nitrocli.
#[derive(Debug)]
pub enum Command {
  Clear,
  Close,
  Open,
  Status,
}

impl Command {
  /// Execute this command with the given arguments.
  pub fn execute(&self, args: Vec<String>) -> Result<()> {
    match *self {
      Command::Clear => clear(args),
      Command::Close => close(args),
      Command::Open => open(args),
      Command::Status => status(args),
    }
  }
}

impl fmt::Display for Command {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match *self {
        Command::Clear => "clear",
        Command::Close => "close",
        Command::Open => "open",
        Command::Status => "status",
      }
    )
  }
}

impl FromStr for Command {
  type Err = ();

  fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
    match s {
      "clear" => Ok(Command::Clear),
      "close" => Ok(Command::Close),
      "open" => Ok(Command::Open),
      "status" => Ok(Command::Status),
      _ => Err(()),
    }
  }
}

/// Invoke the given parser on the given arguments and handles the result.
///
/// This macro invokes the given argument parser on the given arguments
/// using the standard output and error.  It then handles three cases:
/// If the result is `Ok`, it does nothing as the arguments have been
/// parsed successfully.  If the result is `Err(0)`, it returns `Ok(())`
/// as the help message has been printed by `argparse`.  Otherwise it
/// returns `Err(Error::ArgparseError)` as `argparse` failed.
macro_rules! parse_args {
  ( $p:expr, $a:expr ) => {
    if let Err(err) = $p.parse($a, &mut io::stdout(), &mut io::stderr()) {
      match err {
        0 => return Ok(()),
        _ => return Err(Error::ArgparseError),
      }
    }
  };
}

/// Inquire the status of the nitrokey.
fn status(args: Vec<String>) -> Result<()> {
  let mut parser = argparse::ArgumentParser::new();
  parser.set_description("Print the status of the connected Nitrokey Storage");
  parse_args!(parser, args);

  commands::status()
}

/// Open the encrypted volume on the nitrokey.
fn open(args: Vec<String>) -> Result<()> {
  let mut parser = argparse::ArgumentParser::new();
  parser.set_description("Opens the encrypted volume on a Nitrokey Storage");
  parse_args!(parser, args);

  commands::open()
}

/// Close the previously opened encrypted volume.
fn close(args: Vec<String>) -> Result<()> {
  let mut parser = argparse::ArgumentParser::new();
  parser.set_description("Closes the encrypted volume on a Nitrokey Storage");
  parse_args!(parser, args);

  commands::close()
}

/// Clear the PIN stored when opening the nitrokey's encrypted volume.
fn clear(args: Vec<String>) -> Result<()> {
  let mut parser = argparse::ArgumentParser::new();
  parser.set_description("Clears the cached passphrase");
  parse_args!(parser, args);

  commands::clear()
}

/// Parse the command-line arguments and return the selected command and
/// the remaining arguments for the command.
pub fn parse_arguments() -> (Command, Vec<String>) {
  let mut command = Command::Status;
  let mut args = vec![];
  let mut parser = argparse::ArgumentParser::new();
  parser.set_description("Provides access to a Nitrokey device");
  let _ = parser.refer(&mut command).required().add_argument(
    "command",
    argparse::Store,
    "The command to execute (clear|close|open|status)",
  );
  let _ = parser.refer(&mut args).add_argument(
    "arguments",
    argparse::List,
    "The arguments for the command",
  );
  parser.stop_on_first_argument(true);
  parser.parse_args_or_exit();
  drop(parser);

  args.insert(0, format!("nitrocli {}", command));
  (command, args)
}
