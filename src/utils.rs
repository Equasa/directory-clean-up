use std::io::{self, Result, Write};
use std::path::PathBuf;
use std::{fs, path::Path};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self},
};

pub fn ls(path: &Path) -> Result<(Vec<PathBuf>, Vec<PathBuf>)> {
    let mut files: Vec<PathBuf> = Vec::new();
    let mut directories: Vec<PathBuf> = Vec::new();

    match fs::read_dir(path) {
        Ok(paths) => {
            for path in paths {
                let real_path = path.unwrap().path();
                if real_path.is_dir() {
                    directories.push(real_path);
                } else {
                    files.push(real_path)
                }
            }
        }
        Err(e) => {
            println!("Error occured when reading directory: {e}");
            return Err(e);
        }
    }

    return Ok((files, directories));
}

pub fn confirm(prompt: &str) -> io::Result<bool> {
    let mut stdout = io::stdout();

    terminal::enable_raw_mode()?;

    execute!(stdout, cursor::MoveToColumn(0))?;
    print!("{} [y/n] ", prompt);
    stdout.flush()?;

    let answer = loop {
        // Vänta på något inmatas
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char('y') | KeyCode::Char('Y') => break true,
                KeyCode::Char('n') | KeyCode::Char('N') => break false,
                KeyCode::Char('q') | KeyCode::Char('Q') => std::process::exit(0),
                _ => {} 
            }
        }
    };

    terminal::disable_raw_mode()?;
    println!();
    Ok(answer)
}

pub fn delete_object(path: &Path, is_file: bool) -> Result<()> {
    if is_file {
        return fs::remove_file(path);
    } else {
        return fs::remove_dir(path);
    }
}
