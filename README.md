# Rust CLI Animator

Rust CLI Animator is a command-line tool for creating, editing, and playing ASCII animations directly in your terminal. This project demonstrates how to build an interactive CLI application in Rust, featuring a user-friendly interface for managing animations.

## Features

- Create and edit ASCII art animations
- Play animations with adjustable speed
- Add, edit, delete, and reorder frames
- Save animations to files and load them later
- Interactive frame editor with line-by-line editing
- Simple and intuitive menu-driven interface

## Prerequisites

To run this project, you need to have Rust and Cargo installed on your system. If you haven't installed Rust yet, you can do so by following the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

## Installation

1. Clone this repository:
   ```
   git clone https://github.com/yourusername/rust-cli-animator.git
   cd rust-cli-animator
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. Run the animator:
   ```
   cargo run --release
   ```

## Usage

When you run the animator, you'll be presented with a main menu offering the following options:

1. Play animation
2. Edit current frame
3. Add new frame
4. Delete current frame
5. Reorder frames
6. Adjust speed
7. Save animation
8. Load animation
q. Quit

Use the number keys to select an option, or 'q' to quit the program.

### Creating an Animation

1. Start by adding a new frame (option 3).
2. Use the frame editor to create your ASCII art.
3. Add more frames as needed.
4. Use the play animation option (1) to preview your work.

### Editing Frames

In the frame editor:
- Use 'a' to add a new line
- Use 'e <line_number>' to edit a specific line
- Use 'd <line_number>' to delete a line
- Use 'q' to finish editing and return to the main menu

### Saving and Loading Animations

- Use option 7 to save your animation to a file.
- Use option 8 to load a previously saved animation.

## Contributing

Contributions to the Rust CLI Animator are welcome! Please feel free to submit pull requests, create issues, or suggest new features.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- The Rust community for providing excellent documentation and crates.
- The creators and maintainers of the `crossterm` and `serde` crates, which are essential to this project.
