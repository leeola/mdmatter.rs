extern crate mdmatter;

use std::io::Read;
use std::fs::File;

fn main() {
    // First, open the file and read it into a string.
    let mut md_file = File::open("./examples/basic.md").unwrap();
    let mut file_content = String::new();
    md_file.read_to_string(&mut file_content).unwrap();

    // Next, pass the &str into the parser to get back our frontmatter and content.
    let (frontmatter, markdown) = mdmatter::codeblock_matter(&*file_content).unwrap();

    println!("\nFrontmatter:\n{}\n", frontmatter);
    println!("\nMarkdown:\n{}\n", markdown);
}
