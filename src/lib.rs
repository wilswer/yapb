use pyo3::prelude::*;
use std::io::{self, Write};

fn replace_nth_char_ascii(s: &mut str, idx: usize, newchar: char) {
    let s_bytes: &mut [u8] = unsafe { s.as_bytes_mut() };
    assert!(idx < s_bytes.len());
    assert!(s_bytes[idx].is_ascii());
    assert!(newchar.is_ascii());
    // we've made sure this is safe.
    s_bytes[idx] = newchar as u8;
}
fn replace_range_nth_char_ascii(s: &mut str, idx: usize, newchar: char) {
    let s_bytes: &mut [u8] = unsafe { s.as_bytes_mut() };
    assert!(idx < s_bytes.len());
    assert!(newchar.is_ascii());
    for i in 0..idx {
        assert!(s_bytes[i].is_ascii());
        s_bytes[i] = newchar as u8;
    }
}

fn format_bar_push(
    message: &str,
    bar: &str,
    current: usize,
    length: usize,
    description: &str,
) -> String {
    let current_str = current.to_string();
    let length_str = length.to_string();
    let mut out = String::with_capacity(
        6 + message.len() + bar.len() + current_str.len() + length_str.len() + description.len(),
    );
    out.push_str(message);
    out.push(' ');
    out.push('[');
    out.push_str(bar);
    out.push(']');
    out.push(' ');
    out.push_str(&current_str);
    out.push('/');
    out.push_str(&length_str);
    out.push(' ');
    out.push_str(description);
    out
}

#[pymodule]
fn yapb(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ProgressBar>()?;
    Ok(())
}

#[pyclass]
pub struct ProgressBar {
    length: usize,
    current: usize,
    message: String,
    description: String,
    character: char,
    head: char,
    done: bool,
    max_length: usize,
    bar: String,
}

#[pymethods]
impl ProgressBar {
    #[new]
    pub fn new(
        length: usize,
        message: String,
        description: String,
        max_length: Option<usize>,
    ) -> ProgressBar {
        ProgressBar {
            length,
            current: 0,
            message,
            description,
            character: '=',
            head: '>',
            done: false,
            max_length: max_length.unwrap_or(50),
            bar: ' '.to_string().repeat(max_length.unwrap_or(50)),
        }
    }

    pub fn update(&mut self, message: Option<String>, description: Option<String>) {
        if self.done {
            return;
        }
        self.current += 1;
        if self.current >= self.length {
            self.done = true;
        }
        if message.is_some() {
            self.message = message.unwrap();
        }
        if description.is_some() {
            self.description = description.unwrap();
        }
    }

    pub fn render(&mut self) {
        let effective_current =
            ((self.current as f64 / self.length as f64) * (self.max_length as f64)) as usize;
        if self.length > self.max_length {
            if !self.done {
                if effective_current > 0 {
                    replace_nth_char_ascii(&mut self.bar, effective_current - 1, self.character);
                }
                replace_nth_char_ascii(&mut self.bar, effective_current, self.head);
            } else {
                replace_nth_char_ascii(&mut self.bar, self.max_length - 1, self.character);
            }
        } else {
            if !self.done {
                if effective_current > 0 {
                    replace_range_nth_char_ascii(&mut self.bar, effective_current, self.character);
                }
                replace_nth_char_ascii(&mut self.bar, effective_current, self.head);
            } else {
                replace_range_nth_char_ascii(&mut self.bar, self.max_length - 1, self.character);
                replace_nth_char_ascii(&mut self.bar, self.max_length - 1, self.character);
            }
        }
        if !self.done {
            print!(
                "\r{}",
                format_bar_push(
                    &self.message,
                    &self.bar,
                    self.current,
                    self.length,
                    &self.description
                )
            );
            io::stdout().flush().unwrap();
        } else {
            println!(
                "\r{}",
                format_bar_push(
                    &self.message,
                    &self.bar,
                    self.current,
                    self.length,
                    &self.description
                )
            );
        }
    }
}
