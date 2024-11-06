<h1 align=center >
  <img alt="Ferris" src="./assets/ferris.svg">
  <br>
  smart-road
</h1>

## Table of Contents
- [Overview](#overview)
- [Tech Stack](#tech-stack)
- [Installation](#installation)
    - [Cloning](#cloning)
    - [File structure](#file-structure)
    - [Blueprint](#blueprint)
    - [SDL2 library](#sdl2-library)
        - [Description](#description)
        - [Installing](#installing)
- [Usage](#usage)
    - [Initialization](#initialization)
    - [Window](#window)
    - [Events](#events)
    - [About](#about)
- [Interface](#interface)
- [Contributors](#contributors)
- [License](#license)

## Overview

                   |   |   |   |   |   |   |
                   |   |   |   |   |   |   |
                   |   |   |   |   |   |   |
                   |r  | s | l |   |   |   |
    _______________| ← | ↓ | → |   |   |   |________________
                               |            ↑ r
    _______________            |            ________________
                               |            ← s
    _______________            |            ________________
                               |            ↓ l
    ___________________________|____________________________
               l ↑             |
    _______________            |            ________________
               s →             |
    _______________            |            ________________
               r ↓             |
    _______________            |            ________________
                   |   |   |   | ← | ↑ | → |
                   |   |   |   | l | s | r |
                   |   |   |   |   |   |   |
                   |   |   |   |   |   |   |
                   |   |   |   |   |   |   |
                   |   |   |   |   |   |   |

###### [_Table of Contents ⤴️_](#table-of-contents)

## Tech Stack

[![RUST](https://img.shields.io/badge/Rust-black?style=for-the-badge&logo=rust&logoColor=#E57324)](./src/main.rs)
[![SHELL SCRIPT](https://img.shields.io/badge/Shell_Script-121011?style=for-the-badge&logo=gnu-bash&logoColor=white)](./gitify.sh)
[![MARKDOWN](https://img.shields.io/badge/Markdown-000000?style=for-the-badge&logo=markdown&logoColor=white)](#table-of-contents)
[![GITHUB](https://img.shields.io/badge/GitHub-100000?style=for-the-badge&logo=github&logoColor=white)](https://github.com/D4rkJvck/smart-road.git)
[![WARP](https://img.shields.io/badge/warp-01A4FF?style=for-the-badge&logo=warp&logoColor=white)]()
[![LINUX](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)]()
[![MAC OS](https://img.shields.io/badge/mac%20os-000000?style=for-the-badge&logo=apple&logoColor=white)]()


## Installation

### Cloning

To clone the repository, use the following command:

```shell
$ git clone https://github.com/Zone01Dakar/smart-road.git
$ cd smart-road
$ tree
$ code .
```

### File Structure
    .
    |
    +-- assets/
    |       |
    |       +---- cars/
    |       |       |
    |       |       + black.png
    |       |       + police.png
    |       |       + red.png
    |       |       + taxi.png
    |       |
    |       + ferris.svg
    |       + road.jpeg    
    |
    +----- src/
    |       |
    |       +-- controllers/
    |       |       |
    |       |       + mod.rs
    |       |       + stats.rs
    |       |
    |       +-- models/
    |       |       |
    |       |       +-- vehicle/
    |       |       |       |
    |       |       |       + actions.rs
    |       |       |       + attributes.rs
    |       |       |       + enums.rs
    |       |       |       + mod.rs
    |       |       |       + state.rs
    |       |       |
    |       |       + mod.rs
    |       |
    |       +--- views/
    |       |       |
    |       |       + event.rs
    |       |       + mod.rs
    |       |       + render.rs
    |       |
    |       + lib.rs
    |       + macros.rs
    |       + main.rs
    |       + utils.rs
    |
    +--- todos/
    |       |
    |       + audit.todo
    |       + tasks.todo
    |
    + .gitignore
    + Cargo.lock
    + Cargo.toml
    + gitify.sh
    + LICENSE
    + README.md

###### [_Table of Contents ⤴️_](#table-of-contents)

### SDL2 Library

Before running the program, the sdl2 library needs to be installed.

#### Description

SDL2 (Simple DirectMedia Layer) is a multimedia library that provides a simple interface for managing graphics, sound, user input, and other elements necessary for developing games and multimedia applications. It is widely used to create applications that require low-level access to display, audio, and input devices.

#### Installing

Make sure the SDL2 libraries is intalled on the system. The installation instructions may vary depending on the operating system:

- **On Ubuntu/Debian:**
```shell
$ sudo apt-get install libsdl2-dev
```

- **On macOS:**
```shell
$ brew install sdl2
```

To use SDL2 in a Rust project, the `sdl2` dependency needs to be added to the `Cargo.toml` file:

```toml
[dependencies]
sdl2 = "0.34" // Check for later verions
```

###### [_Table of Contents ⤴️_](#table-of-contents)

## Usage

### Initialization
Firstly, the sdl2 needs to be initialized using the **`init()`** function of the **`sdl2`** library to get a subsystem layer context.  
Then the video subsystem responsible for managing the windows and rendering is retrieved from the resulting context via the **`video()`** method.  
At this point the program has everything it needs to create windows and canvas.

### Window

A window can be initialized using the **`window()`** method on the video subsystem retrived from the subsystem context. The method takes the specified title and dimensions as arguments. To center the window on the screen, there is the **`position_centered()`** method. then the **`build()`** method is used to create the window.  
And finally, the window needs to be transformed into a canvas for graphical rendering via the **`into_canvas()`** method on the window intance.

### Events

Some events are to be handled to provide a user interaction.  
To begin with a event pump is initialized from the context using the **`event_pump()`** method. This event pump act like a pipeline transferring the user input events from the sdl context events queue to the program.  

An infinite loop si then triggered to run the program as long as the user does not close the window.  
Within the loop, the program can continuously listen to input events and handle them.  
Some of those input events can be:
- **`Event::Quit`**: Triggered when the user tries to close the window.
- **`Event::KeyDown`**: Detects if a key is pressed, and here we specifically check if the **Escape** key was pressed to exit the loop.

On each iteration the canvas is cleared via the **`clear()`** method and the display is updated with the current rendering of the canvas via the **`present()`** method in a time interval that will be set with the **`std::thread::sleep()`** function that will limit the number of frames per second to 60 using a delay.

### About

SDL2 is a powerful and flexible library that simplifies the development of multimedia applications by providing low-level abstractions for display, audio, and input. By using Rust with SDL2, you can leverage Rust's memory safety and performance while developing games or interactive applications.

For more in-depth information, refer to the [official SDL2 documentation](https://wiki.libsdl.org/) and [crates.io for sdl2](https://crates.io/crates/sdl2) for more examples and available features.

###### [_Table of Contents ⤴️_](#table-of-contents)

## Interface

To open a new window, run the program.

```rust
$ cargo run
   Compiling bitflags v1.3.2
   Compiling lazy_static v1.5.0
   Compiling libc v0.2.161
   Compiling sdl2-sys v0.37.0
   Compiling sdl2 v0.37.0
   Compiling smart-road v0.1.0 (/home/student/smart-road)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.20s
     Running `target/debug/smart-road`
```

## Contributors

[![jefaye](https://shields.io/badge/jefaye-Zone01-blue)](http://learn.zone01dakar.sn/git/jefaye)

## License

[![MIT](https://shields.io/badge/License-MIT-yellow)](LICENSE)