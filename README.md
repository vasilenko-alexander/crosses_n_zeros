<h1 align="center">
    <a href="https://github.com/vasilenko-alexander/crosses_n_zeros">
        <img src="docs/images/tic-tac-toe-game-svgrepo-com.svg" width="125" height="125" />
    </a>
</h1>

# Welcome to "Cross vs Zero"

## About
**Cross vs Zero** is a simple tic-tac-toe game that is written in **Rust**

## Getting Start

### Prerequisites

To build and run **"Cross vs Zero"** you will need **Rust** installed on your machine. Check official [Rust installation guide](https://www.rust-lang.org/tools/install) how to do it for your platform.

### Setup

#### Download
You can download it directly by using following link [download](https://github.com/vasilenko-alexander/crosses_n_zeros/archive/main.zip)

Or you can run next command in terminal:
```
curl --proto "=https" -L -O "https://github.com/vasilenko-alexander/crosses_n_zeros/archive/main.zip"
```
Now you need to unpack it to a directory of your choice and go to that directory.

#### Build

To build **Cross vs Zero** you need to run next command in terminal from project's root directory

```
cargo build --release --workspace
```

#### Run

Now that you have built **Cross vs Zero**, it's time to run it. You have to options here:

- First, run the binary that is located in 

```
{project's root directory}/target/release
```

- Second, run following command in terminal from project's root directory

```
cargo run --package cross_vs_zero
```

### Have fun playing it!

## License

This project is licensed under **MIT license**.

See [LICENSE](LICENSE) for more information.