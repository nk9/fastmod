/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use anyhow::Result;
use crossterm::style::SetForegroundColor;
use crossterm::style::SetAttribute;
use crossterm::style::ResetColor;
use crossterm::style::Color;
use crossterm::style::Attribute;
use similar::{ChangeTag, TextDiff};


pub fn clear() {
    #[cfg(not(test))]
    {
        use std::io::stdout;
        use crossterm::execute;
        use crossterm::terminal::{Clear, ClearType};

        if execute!(stdout(), Clear(ClearType::All)).is_err() {
            print!("{}", "\n".repeat(8));
        }
    }
    #[cfg(test)]
    {
        // During tests, just print newlines
        println!("{}", "\n".repeat(8));
    }
}

pub fn size() -> Option<(usize, usize)> {
    crossterm::terminal::size()
        .ok()
        .map(|(w, h)| (w as usize, h as usize))
}

pub fn print_bolded_diff_to_terminal(before: &str, after: &str) -> Result<()> {
    let diff = TextDiff::from_chars(before, after);

    print!("{}- ", SetForegroundColor(Color::DarkRed));
    for op in diff.ops() {
        for change in diff.iter_changes(op) {
            match change.tag() {
                ChangeTag::Equal => {
                    print!("{}", change.value());
                }
                ChangeTag::Delete => {
                    print!("{}{}{}{}",
                        SetAttribute(Attribute::Bold),
                        change.value(),
                        SetAttribute(Attribute::Reset),
                        SetForegroundColor(Color::DarkRed),
                    );
                }
                ChangeTag::Insert => { /* Only show deleted chars for "before" line */ }
            }
        }
    }
    print!("{}\n", ResetColor);

    print!("{}+ ", SetForegroundColor(Color::DarkGreen));
    for op in diff.ops() {
        for change in diff.iter_changes(op) {
            match change.tag() {
                ChangeTag::Equal => {
                    print!("{}", change.value());
                }
                ChangeTag::Insert => {
                    print!("{}{}{}{}",
                        SetAttribute(Attribute::Bold),
                        change.value(),
                        SetAttribute(Attribute::Reset),
                        SetForegroundColor(Color::DarkGreen),
                    );
                }
                ChangeTag::Delete => { /* Only show added chars for "after" line */ }
            }
        }
    }
    print!("{}\n", ResetColor);

    Ok(())
}
