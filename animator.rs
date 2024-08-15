use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    ExecutableCommand,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{stdout, Write};
use std::time::Duration;

#[derive(Clone, Serialize, Deserialize)]
struct Frame {
    content: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Animation {
    frames: Vec<Frame>,
    current_frame: usize,
    speed: u64,
}

impl Animation {
    fn new() -> Self {
        Animation {
            frames: vec![],
            current_frame: 0,
            speed: 500,
        }
    }

    fn add_frame(&mut self, frame: Frame) {
        self.frames.push(frame);
    }

    fn next_frame(&mut self) -> &Frame {
        let frame = &self.frames[self.current_frame];
        self.current_frame = (self.current_frame + 1) % self.frames.len();
        frame
    }

    fn insert_frame(&mut self, index: usize, frame: Frame) {
        self.frames.insert(index, frame);
    }

    fn delete_frame(&mut self, index: usize) {
        if self.frames.len() > 1 {
            self.frames.remove(index);
            self.current_frame = self.current_frame.min(self.frames.len() - 1);
        }
    }

    fn move_frame(&mut self, from: usize, to: usize) {
        if from < self.frames.len() && to < self.frames.len() {
            let frame = self.frames.remove(from);
            self.frames.insert(to, frame);
        }
    }
}

fn main() -> crossterm::Result<()> {
    let mut animation = Animation::new();
    let mut stdout = stdout();

    // Example animation
    animation.add_frame(Frame {
        content: vec![
            "  o  ".to_string(),
            " /|\\ ".to_string(),
            " / \\ ".to_string(),
        ],
    });
    animation.add_frame(Frame {
        content: vec![
            "  o  ".to_string(),
            " /|\\ ".to_string(),
            " | | ".to_string(),
        ],
    });

    enable_raw_mode()?;

    loop {
        stdout.execute(Clear(ClearType::All))?;

        let frame = animation.next_frame();
        for line in &frame.content {
            println!("{}", line);
        }

        println!("\nMain Menu:");
        println!("1. Play animation");
        println!("2. Edit current frame");
        println!("3. Add new frame");
        println!("4. Delete current frame");
        println!("5. Reorder frames");
        println!("6. Adjust speed (current: {}ms)", animation.speed);
        println!("7. Save animation");
        println!("8. Load animation");
        println!("q. Quit");

        stdout.flush()?;

        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char('1') => play_animation(&animation)?,
                KeyCode::Char('2') => edit_frame(&mut animation)?,
                KeyCode::Char('3') => add_new_frame(&mut animation)?,
                KeyCode::Char('4') => delete_current_frame(&mut animation)?,
                KeyCode::Char('5') => reorder_frames(&mut animation)?,
                KeyCode::Char('6') => adjust_speed(&mut animation)?,
                KeyCode::Char('7') => save_animation(&animation)?,
                KeyCode::Char('8') => {
                    animation = load_animation()?;
                }
                KeyCode::Char('q') => break,
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}

fn play_animation(animation: &Animation) -> crossterm::Result<()> {
    let mut stdout = stdout();
    enable_raw_mode()?;

    for _ in 0..50 {
        stdout.execute(Clear(ClearType::All))?;
        let frame = &animation.frames[animation.current_frame];
        for line in &frame.content {
            println!("{}", line);
        }
        stdout.flush()?;

        if event::poll(Duration::from_millis(animation.speed))? {
            break;
        }
    }

    disable_raw_mode()?;
    Ok(())
}

fn edit_frame(animation: &mut Animation) -> crossterm::Result<()> {
    let mut stdout = stdout();
    stdout.execute(Clear(ClearType::All))?;

    let frame = &mut animation.frames[animation.current_frame];
    
    println!("Editing current frame. Commands:");
    println!("'a' to add a line, 'e <line_number>' to edit a line, 'd <line_number>' to delete a line");
    println!("'q' to finish editing");

    loop {
        for (i, line) in frame.content.iter().enumerate() {
            println!("{}: {}", i, line);
        }

        let mut command = String::new();
        std::io::stdin().read_line(&mut command)?;
        let parts: Vec<&str> = command.trim().split_whitespace().collect();

        match parts.get(0) {
            Some(&"a") => {
                println!("Enter new line:");
                let mut new_line = String::new();
                std::io::stdin().read_line(&mut new_line)?;
                frame.content.push(new_line.trim().to_string());
            }
            Some(&"e") => {
                if let Some(line_num) = parts.get(1).and_then(|s| s.parse::<usize>().ok()) {
                    if line_num < frame.content.len() {
                        println!("Enter new content for line {}:", line_num);
                        let mut new_line = String::new();
                        std::io::stdin().read_line(&mut new_line)?;
                        frame.content[line_num] = new_line.trim().to_string();
                    }
                }
            }
            Some(&"d") => {
                if let Some(line_num) = parts.get(1).and_then(|s| s.parse::<usize>().ok()) {
                    if line_num < frame.content.len() {
                        frame.content.remove(line_num);
                    }
                }
            }
            Some(&"q") => break,
            _ => println!("Invalid command"),
        }

        stdout.execute(Clear(ClearType::All))?;
    }

    Ok(())
}

fn add_new_frame(animation: &mut Animation) -> crossterm::Result<()> {
    let mut stdout = stdout();
    stdout.execute(Clear(ClearType::All))?;

    println!("Adding a new frame. Enter content (empty line to finish):");
    
    let mut new_content = vec![];
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line)?;
        if line.trim().is_empty() {
            break;
        }
        new_content.push(line.trim_end().to_string());
    }

    if !new_content.is_empty() {
        animation.add_frame(Frame { content: new_content });
    }

    Ok(())
}

fn delete_current_frame(animation: &mut Animation) -> crossterm::Result<()> {
    animation.delete_frame(animation.current_frame);
    Ok(())
}

fn reorder_frames(animation: &mut Animation) -> crossterm::Result<()> {
    let mut stdout = stdout();
    stdout.execute(Clear(ClearType::All))?;

    println!("Current frame order:");
    for (i, frame) in animation.frames.iter().enumerate() {
        println!("{}. {} lines", i, frame.content.len());
    }

    println!("\nEnter the frame number to move, followed by its new position:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let parts: Vec<usize> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    if parts.len() == 2 {
        animation.move_frame(parts[0], parts[1]);
    }

    Ok(())
}

fn adjust_speed(animation: &mut Animation) -> crossterm::Result<()> {
    println!("Enter new speed in milliseconds:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    if let Ok(new_speed) = input.trim().parse() {
        animation.speed = new_speed;
    }
    Ok(())
}

fn save_animation(animation: &Animation) -> crossterm::Result<()> {
    println!("Enter filename to save:");
    let mut filename = String::new();
    std::io::stdin().read_line(&mut filename)?;
    let filename = filename.trim();

    let serialized = serde_json::to_string(animation).unwrap();
    fs::write(filename, serialized)?;
    println!("Animation saved to {}", filename);
    Ok(())
}

fn load_animation() -> crossterm::Result<Animation> {
    println!("Enter filename to load:");
    let mut filename = String::new();
    std::io::stdin().read_line(&mut filename)?;
    let filename = filename.trim();

    let contents = fs::read_to_string(filename)?;
    let animation: Animation = serde_json::from_str(&contents).unwrap();
    println!("Animation loaded from {}", filename);
    Ok(animation)
}
