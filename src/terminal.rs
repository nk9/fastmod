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

use std::io::stdout;

use anyhow::Result;
use crossterm::execute;
use crossterm::style::SetForegroundColor;
use crossterm::style::SetAttribute;
use crossterm::style::ResetColor;
use crossterm::style::Color;
use crossterm::style::Attribute;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use similar::{ChangeTag, TextDiff};


pub fn clear() {
    if execute!(stdout(), Clear(ClearType::All)).is_err() {
        print!("{}", "\n".repeat(8));
    }
}

pub fn size() -> Option<(usize, usize)> {
    crossterm::terminal::size()
        .ok()
        .map(|(w, h)| (w as usize, h as usize))
}

pub fn print_bolded_diff_to_terminal(before: &str, after: &str) -> Result<()> {
    let mut stdout = stdout();
    let diff = TextDiff::from_chars(before, after);

    execute!(stdout, SetForegroundColor(Color::DarkRed), Print("- "))?;
    for op in diff.ops() {
        for change in diff.iter_changes(op) {
            match change.tag() {
                ChangeTag::Equal => {
                    execute!(stdout, Print(change.value()))?;
                }
                ChangeTag::Delete => {
                    execute!(
                        stdout,
                        SetAttribute(Attribute::Bold),
                        Print(change.value()),
                        SetAttribute(Attribute::Reset),
                        SetForegroundColor(Color::DarkRed),
                    )?;
                }
                ChangeTag::Insert => { /* Only show deleted chars for "before" line */ }
            }
        }
    }
    execute!(stdout, ResetColor, Print("\n"))?;

    execute!(stdout, SetForegroundColor(Color::DarkGreen), Print("+ "))?;
    for op in diff.ops() {
        for change in diff.iter_changes(op) {
            match change.tag() {
                ChangeTag::Equal => {
                    execute!(stdout, Print(change.value()))?;
                }
                ChangeTag::Insert => {
                    execute!(
                        stdout,
                        SetAttribute(Attribute::Bold),
                        Print(change.value()),
                        SetAttribute(Attribute::Reset),
                        SetForegroundColor(Color::DarkGreen),
                    )?;
                }
                ChangeTag::Delete => { /* Only show added chars for "after" line */ }
            }
        }
    }
    execute!(stdout, ResetColor, Print("\n"))?;

    Ok(())
}
