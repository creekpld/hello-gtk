# example GTK app with Rust

## Acknowledgment 
[Source](https://www.youtube.com/watch?v=hLR8R0Zl0yY)

## Requirements
[GTK-RS Rust Bindings](http://gtk-rs.org/docs/requirements.html)


## Setup

   ```sh
    # 1.install rust nightly with rustup
    curl https://sh.rustup.rs -sSf | sh

    # 2. update 
    rustup update

    # 3. Install the Rust language Server (only for development)
    rustup component add rls-preview
    rustup component add rust-analysis
    rustup component add rust-src
  ```

## Run

### bash
  ```sh
    git clone https://github.com/creekpld/hello-gtk.git && /
    cd hello-gtk && /
    cargo run
  ```
### fish
  ```sh
    git clone https://github.com/creekpld/hello-gtk.git; cd hello-gtk; cargo run
  ```