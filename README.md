
# mdmatter (unstable)

mdmatter is a simple Rust library for extracting front matter from a markdown
file.

Currently, this library only scans for the triple backtick. Eg:

    ```

And not:

    ---

## Usage

See [./examples/basic.rs](./examples/basic.rs) for example usage. Run with:

    cargo run --example basic

## TODO

- Scan for classic `---` frontmatter tags as well.
