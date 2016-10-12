#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]

#[macro_use]
extern crate nom;

pub use nom::IResult;

#[derive(Debug,PartialEq,Eq)]
pub enum Error {
    // Any failure to parsing the initial frontmatter will result in a no matter
    // failure.
    NoMetadata,
}

named!(codeblock_matter_parser (&str) -> &str,
       delimited!(tag_s!("```\n"),
  take_until_s!("\n```"),
  tag_s!("\n```\n")));

pub fn codeblock_matter(data: &str) -> Result<(&str, &str), Error> {
    match codeblock_matter_parser(data) {
        IResult::Done(content, frontmatter) => Ok((frontmatter, content)),
        // TODO(leeola): Don't drop the context here with the generic NoMetadata
        // error.
        //
        // I'm currently dropping due to prototyping, and i'm not sure how i want to
        // generically correleate the IResult information to a mdmatter caller.
        IResult::Incomplete(_) => Err(Error::NoMetadata),
        IResult::Error(_) => Err(Error::NoMetadata),
    }
}

#[cfg(test)]
mod tests {
    pub use super::codeblock_matter;
    pub use nom::IResult::Done;

    pub fn format_codeblock(matter: &str, content: &str) -> String {
        format!("```\n{}\n```\n{}", matter, content)
    }

    describe! codeblocks {
        before_each {
            let matter_1 = "matter";
            let content_1 = "not matter";
            let codeblock_1 = format_codeblock(matter_1, content_1);

            let matter_2 = "title: foo
template: bar
some_list:
    - 1
    - 2
";
            let content_2 = "

fuller content example
over multiple lines.
";
            let codeblock_2 = format_codeblock(matter_2, content_2);
        }

        it "should be parsed by codeblock_matter" {
            assert_eq!(codeblock_matter(&*codeblock_1), Ok((matter_1, content_1)));
            assert_eq!(codeblock_matter(&*codeblock_2), Ok((matter_2, content_2)));
        }

        it "should be parsed by codeblock_matter_parser" {
            use super::super::codeblock_matter_parser;
            // nom returns the values in reverse compared to mdmatter
            assert_eq!(codeblock_matter_parser(&*codeblock_1), Done(content_1, matter_1));
            assert_eq!(codeblock_matter_parser(&*codeblock_2), Done(content_2, matter_2));
        }
    }
}
