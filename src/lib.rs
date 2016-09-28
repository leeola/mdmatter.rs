#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]

#[macro_use]
extern crate nom;

pub use nom::IResult;

#[derive(Debug,PartialEq,Eq)]
pub enum Error {
    // Any failure to parsing the initial markdown-meta will result in a no meta
    // failure.
    NoMetadata,
}

named!(codeblock_meta_parser (&str) -> &str,
       delimited!(tag_s!("```\n"),
  take_until_s!("\n```"),
  tag_s!("\n```\n")));

pub fn codeblock_meta(data: &str) -> Result<(&str, &str), Error> {
    match codeblock_meta_parser(data) {
        IResult::Done(not_meta, meta) => Ok((meta, not_meta)),
        // TODO(leeola): Don't drop the context here with the generic NoMetadata
        // error.
        //
        // I'm currently dropping due to prototyping, and i'm not sure how to
        // generically correleate the IResult information to a markdown-meta user.
        IResult::Incomplete(_) => Err(Error::NoMetadata),
        IResult::Error(_) => Err(Error::NoMetadata),
    }
}

#[cfg(test)]
mod tests {
    pub use super::codeblock_meta;
    pub use nom::IResult::Done;

    pub fn format_codeblock(meta: &str, content: &str) -> String {
        format!("```\n{}\n```\n{}", meta, content)
    }

    describe! codeblocks {
        before_each {
            let meta_1 = "meta";
            let content_1 = "not meta";
            let codeblock_1 = format_codeblock(meta_1, content_1);

            let meta_2 = "title: foo
template: bar
some_list:
    - 1
    - 2
";
            let content_2 = "

fuller content example
over multiple lines.
";
            let codeblock_2 = format_codeblock(meta_2, content_2);
        }

        it "should be parsed by codeblock_meta" {
            assert_eq!(codeblock_meta(&*codeblock_1), Ok((meta_1, content_1)));
            assert_eq!(codeblock_meta(&*codeblock_2), Ok((meta_2, content_2)));
        }

        it "should be parsed by codeblock_meta_parser" {
            use super::super::codeblock_meta_parser;
            // nom returns the values in reverse compared to markdown-meta
            assert_eq!(codeblock_meta_parser(&*codeblock_1), Done(content_1, meta_1));
            assert_eq!(codeblock_meta_parser(&*codeblock_2), Done(content_2, meta_2));
        }
    }
}
