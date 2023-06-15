/*
 * Copyright (C) 2023 gulje <guljeee@yandex.com>
 *
 * This program is free software: you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the Free
 * Software Foundation, either version 3 of the License, or (at your option)
 * any later version.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of  MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for
 * more details.
 *
 * You should have received a copy of the GNU General Public License along with
 * this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use clap::Parser;

mod build;
mod lexer;
mod parser;

#[derive(clap::Parser, Debug)]
#[command(version, about)]
#[command(author = "gulje <guljeee@yandex.com>")]
#[command(help_template = r#"{usage-heading} {usage}
{about-section}Version: {version}
{all-args} {tab}

Copyright (C) 2023 {author}
License GPLv3: GNU GPL version 3 <https://gnu.org/licenses/gpl-3.0.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

Report bugs to: {author} or <https://github.com/gulje/harmonia/issues>"#)]
struct Arguments {}

fn main() {
    let input_code = r#"define renaming implementation my_implementation {
        if p contains "tap" {
            return "UwU"
        } else {
            return "rust and roll"
        }
    }

    hello1
"#;

    let _args = Arguments::parse();

    let mut lexer = lexer::Lexer::new(input_code);
    let tokens = lexer.lex();

    for token in tokens {
        println!("{token:?}");
    }
}
