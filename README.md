# urlencode

A simple command line utility for URL encoding or decoding stdin.
Urlencode doesn't have depencencies and aims to be available on every achitecture.

We currently build for:

- Linux
  - x86_64(musl)
  - aarch64 (musl, gnu)
  - armv7 (gnueabihf, musleabihf)
  - arm (gnueabihf, musleabihf)
- MacOs
  - Apple Silicon
  - Intel.
- Windows
  - x86_64

## Installation

### Download and Install the binary

```bash
$ curl -L YOUR_RELEASE_URL_FROM_GITHUB.tar.gz  -o urlencode.tar.gz
$ tar -xf urlencode.tar.gz
# The following will make the command be globally available in your system
$ mv urlencode /usr/local/bin
```

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
$ echo "Hello World\!" | urlencode
# Output: "Hello%20World%21%0A"   <--- Please note the %0A at the end of the line, 
#                                      this is because "echo" withouth any flags 
#                                      will output a new line at the end of the string
$ echo -n "Hello World\!" | urlencode
# Output: "Hello%20World%21"
```

To decode the input, use the `-d` or `--decode` flag:

```bash
$ echo "Hello%20World%21" | urlencode -d
# Output: "Hello World!"
```

### No-newline

The `--nonewline` flag tells urlencode to avoid adding a new line at the end of the string.
This is the same behavior of the `echo` command, and preserves the output even it's chained multiple times inside
`urlencode`.

```bash
$ echo -n "TEST AAAABBBB" | urlencode | urlencode | urlencode | cat -
# Output TEST%252520AAAABBBB%250A%0A

$ echo -n "TEST AAAABBBB" | urlencode -n | urlencode -n | urlencode -n | cat -
# Output: TEST%252520AAAABBBB  <-- no new line
```


## List of all the commands
```bash
$ urlencode --help
A simple command line utility for URL encoding or decoding stdin.

Usage: urlencode [OPTIONS] [TEXT_TO_PARSE]

Arguments:
  [TEXT_TO_PARSE]  Text to parse, this parameter overrides stdin

Options:
  -d, --decode     Decode stdin instead of encoding
  -n, --nonewline  Do not append a newline to the output
  -h, --help       Print help information
  -V, --version    Print version information
```

## Contributors wanted

This project is only at the beginning, feedback and help is always appreciated.  
Feel free to submit pull requests.  
Thank you in advance!

## License

`urlencode` is licensed under the MIT license. See [LICENSE](https://chat.openai.com/chat/LICENSE) for more details.
