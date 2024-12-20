<h1 align=center >
  <img alt="Ferris" src="./assets/img/ferris.svg">
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
- [Simulation](#simulation)
- [Contributors](#contributors)
- [License](#license)

## Overview

The **smart-road** is a traffic control strategy's simulation without traffic light, but a smart **intersection** with **sensors** distributed over its entire surface.
Those sensors are there to be scan by a **robotaxi** sensor.
[**Robotaxis**](https://en.wikipedia.org/wiki/Self-driving_car) are a promising solution to traffic accidents that might be publicly available in the next decade, and thus traffic issues related to autonomous vehicles are also being extensively investigated.
In this project, **robotaxis** have to pass an intersection without any collisions and with a minimum of traffic congestion.


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
$ tree --dirsfirst
$ code .
```

### File Structure

    .
    |
    +-- assets/
    |       |
    |       +--- fonts/
    |       |       |
    |       |       + Doto-Bold.ttf
    |       |
    |       +---- img/
    |               |
    |               + ferris.svg
    |               + intersection.png
    |               + vehicle.png
    |
    +----- src/
    |       |
    |       +-- controllers/
    |       |       |
    |       |       + event.rs
    |       |       + mod.rs
    |       |       + statistics.rs
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
    |       |       + utils.rs
    |       |
    |       +--- views/
    |       |       |
    |       |       + mod.rs
    |       |       + render.rs
    |       |       + stats.rs
    |       |
    |       + lib.rs
    |       + main.rs
    |
    +--- tests/
    |       |
    |       +-- integration/
    |       |
    |       +---- unit/
    |               |
    |               +-- vehicle/
    |                       |
    |                       +-- attributes/
    |                       |       |
    |                       |       +-- ...
    |                       |
    |                       +--- state/
    |                               |
    |                               + into_area_tests.rs
    |                               + is_visible_tests.rs
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
sdl2 = "0.37.0" // Check for later verions
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

## Simulation

The simulation begin with an empty road intersection with `3` lanes for each road.  
Then whenever the user press the dedicated keyboard key, a vehicle spawns from a direction to a direction depending on the pressed key as follows:

- `UP`: from **South** (`bottom`) to **North** (`top`).
- `RIGHT`: from **West** (`left`) to **East** (`right`).
- `DOWN`: from **North** (`top`) to **South** (`bottom`).
- `LEFT`: from **East** (`right`) to **West** (`left`).
- `R`: from a **Random** direction to the opposite direction.
- `A`: **auto** generate from random direction.
- `S`: toggle vehicles **sensors** visibility.

To open a new window, run the program.

```rust
cargo r
   Compiling libc v0.2.162
   Compiling cfg-if v1.0.0
   Compiling proc-macro2 v1.0.89
   Compiling unicode-ident v1.0.13
   Compiling autocfg v1.4.0
   Compiling version-compare v0.1.1
   Compiling sdl2-sys v0.37.0
   Compiling lock_api v0.4.12
   Compiling byteorder v1.5.0
   Compiling parking_lot_core v0.9.10
   Compiling quote v1.0.37
   Compiling getrandom v0.2.15
   Compiling syn v2.0.87
   Compiling scopeguard v1.2.0
   Compiling smallvec v1.13.2
   Compiling rand_core v0.6.4
   Compiling sdl2 v0.37.0
   Compiling parking_lot v0.12.3
   Compiling mio v1.0.2
   Compiling signal-hook-registry v1.4.2
   Compiling socket2 v0.5.7
   Compiling bitflags v1.3.2
   Compiling lazy_static v1.5.0
   Compiling pin-project-lite v0.2.15
   Compiling bytes v1.8.0
   Compiling zerocopy-derive v0.7.35
   Compiling tokio-macros v2.4.0
   Compiling zerocopy v0.7.35
   Compiling tokio v1.41.1
   Compiling ppv-lite86 v0.2.20
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling smart-road v0.1.0 (/Users/ivan/Desktop/smart-road)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 43.77s
     Running `target/debug/smart-road`
```

## Contributors

### Collaborators

[![jefaye](https://shields.io/badge/jefaye-Zone01-yellow)](http://learn.zone01dakar.sn/git/jefaye)
[![masamba](https://shields.io/badge/masamba-Zone01-yellow)](http://learn.zone01dakar.sn/git/masamba)
[![npouille](https://shields.io/badge/npouille-Zone01-yellow)](http://learn.zone01dakar.sn/git/npouille)
[![papebsow](https://shields.io/badge/papebsow-Zone01-yellow)](http://learn.zone01dakar.sn/git/papebsow)

### Peers

[![lodiouf](https://shields.io/badge/lodiouf-Zone01-blue)](http://learn.zone01dakar.sn/git/lodiouf)
[![preydedy](https://shields.io/badge/preydedy-Zone01-blue)](http://learn.zone01dakar.sn/git/preydedy)
[![eibounda](https://shields.io/badge/eibounda-Zone01-blue)](http://learn.zone01dakar.sn/git/eibounda)
[![nnourdin](https://shields.io/badge/nnourdin-Zone01-blue)](http://learn.zone01dakar.sn/git/nnourdin)

### Auditors

[![bcoulibal](https://shields.io/badge/bcoulibal-Zone01-green)](http://learn.zone01dakar.sn/git/bcoulibal)
[![bsall](https://shields.io/badge/bsall-Zone01-green)](http://learn.zone01dakar.sn/git/bsall)
[![nnourdin](https://shields.io/badge/nnourdin-Zone01-green)](http://learn.zone01dakar.sn/git/nnourdin)
[![annndiaye](https://shields.io/badge/annndiaye-Zone01-green)](http://learn.zone01dakar.sn/git/annndiaye)
[![mandaw](https://shields.io/badge/mandaw-Zone01-green)](http://learn.zone01dakar.sn/git/mandaw)

## License

[![MIT](https://shields.io/badge/License-MIT-black)](LICENSE)
