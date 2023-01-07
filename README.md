# urlencode

A simple command line utility for URL encoding or decoding stdin.

## Installation

### Install the binary

### Build it yourself

To install `urlencode`, clone this repository and build the executable using Cargo:

```bash
git clone https://github.com/your-username/urlencode
cd urlencode
cargo build --release
```

## Usage

To encode the input, pass it to `urlencode` as stdin:

``` bash
$ echo "Hello World!" | urlencode
# Output: "Hello%20World%21"
```

To decode the input, use the `-d` or `--decode` flag:

```bash
$ echo "Hello%20World%21" | urlencode -d
# Output: "Hello World!"
```

You can also use the `-h` or `--help` flag to show the help menu, which lists the available options and a brief description of the utility:

```bash
$ urlencode --help
```


## License

`urlencode` is licensed under the MIT license. See [LICENSE](https://chat.openai.com/chat/LICENSE) for more details.
