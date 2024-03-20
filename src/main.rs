use std::{io::Write, os::windows::fs::FileExt};

use anyhow::{anyhow, Context, Ok, Result};

// enum FileError {
//     InvalidFileExt,
// }

#[derive(PartialEq, Eq)]
enum CharType {
    LineStart,
    Text,
    Link,
    Star,
    Heading,
    Dash,
    Slash,
    Whitespace,
}

const global_css: &str = "<style>
*,*::after,*::before, html, body {
margin:0;
padding:0;
box-sizing: border-box;
}
body{
font-family: 'Courier New', Courier, monospace;
}
</style>";

fn main() -> Result<()> {
    const file_path: &str = "SPEC.md";
    //check if the file is a txt or md file
    if !file_path.ends_with(".txt") && !file_path.ends_with(".md") {
        return Err(anyhow!("file must be .txt or .md"));
    }
    //read the file specified in the argument
    let file_contents = std::fs::read_to_string(file_path)?;
    let mut output = String::with_capacity(file_contents.len());

    output.push_str("<body>");
    output.push_str("<div>");

    for line in file_contents.lines() {
        let mut heading: i8 = 0; // 1: <h1>, 2: <h2>, ..etc. 0: <p>
                                 //above specified variables affect the overall line whereas hyperlinks only affect a part of a line sometimes. Therefore it is not mentioned above.

        let mut line_iter = line.char_indices();
        let mut prev_type = CharType::LineStart;

        while let Some((idx, ch)) = line_iter.next() {
            let mut char_type = CharType::Text;
            match ch {
                '#' => {
                    if prev_type == CharType::LineStart {
                        char_type = CharType::Heading;
                        heading = 1;
                    }
                    if prev_type == CharType::Heading {
                        char_type = CharType::Heading;
                        heading = std::cmp::min(3, heading + 2); // only upto 3 consecutive '#'s together
                    }

                    //skip and dont push to output
                }
                '*' => {
                    char_type = CharType::Star;
                    //skip and dont push to output
                }
                '-' => {
                    //can be a bullet or if next char is also a dash it is a strikethrough
                    char_type = CharType::Dash;
                    //skip and dont push to output
                }
                '/' => {
                    char_type = CharType::Slash;
                    //skip and dont push to output
                }
                ' ' | '\t' => {
                    char_type = CharType::Whitespace;
                    if prev_type == CharType::Heading {
                        let h_tag = format!("<h{}>", heading);
                        output.push_str(&h_tag);
                    }
                }
                _ => {}
            }

            if char_type == CharType::Text || char_type == CharType::Whitespace {
                //rendering place

                output.push(ch);
            }
            prev_type = char_type;
        }
        //add the closing tags for the line
        let h_tag = format!("</h{}>", heading);
        output.push_str(&h_tag);
        output.push_str("<br>");
    }

    output.push_str("</div>");
    output.push_str("</body>");
    // output.push_str(global_css);

    let out_file_path = "out.html";
    let mut out_file = std::fs::File::create(out_file_path)?;
    out_file.write_all(output.as_bytes())?;
    // std::fs::write(out_file_path, &output)?;
    out_file.write_all(global_css.as_bytes())?;
    // println!("\n{output}\n");
    Ok(())
}
