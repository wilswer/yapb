use pyo3::prelude::*;
use pyo3::Python;
use std::io::{self, Write};

#[pymodule]
fn yapb(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ProgressBar>()?;
    Ok(())
}

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
    capacity: usize,
    description: &str,
) -> String {
    let current_str = current.to_string();
    let capacity_str = capacity.to_string();
    let mut out = String::with_capacity(
        6 + message.len() + bar.len() + current_str.len() + capacity_str.len() + description.len(),
    );
    out.push_str(message);
    out.push(' ');
    out.push('[');
    out.push_str(bar);
    out.push(']');
    out.push(' ');
    out.push_str(&current_str);
    out.push('/');
    out.push_str(&capacity_str);
    out.push(' ');
    out.push_str(description);
    out
}

#[pyclass]
pub struct ProgressBar {
    capacity: usize,
    current: usize,
    message: String,
    description: String,
    character: char,
    head: char,
    done: bool,
    ncols: usize,
    bar: String,
}

#[pymethods]
impl ProgressBar {
    #[new]
    pub fn new(
        capacity: usize,
        message: Option<String>,
        description: Option<String>,
        ncols: Option<usize>,
    ) -> ProgressBar {
        ProgressBar {
            capacity,
            current: 0,
            message: message.unwrap_or("".to_string()),
            description: description.unwrap_or("".to_string()),
            character: '=',
            head: '>',
            done: false,
            ncols: ncols.unwrap_or(50),
            bar: ' '.to_string().repeat(ncols.unwrap_or(50)),
        }
    }

    pub fn update(&mut self, message: Option<String>, description: Option<String>) {
        if self.done {
            return;
        }
        self.current += 1;
        if self.current >= self.capacity {
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
            ((self.current as f64 / self.capacity as f64) * (self.ncols as f64)) as usize;
        if self.capacity > self.ncols {
            if !self.done {
                if effective_current > 0 {
                    replace_nth_char_ascii(&mut self.bar, effective_current - 1, self.character);
                }
                replace_nth_char_ascii(&mut self.bar, effective_current, self.head);
            } else {
                replace_nth_char_ascii(&mut self.bar, self.ncols - 1, self.character);
            }
        } else {
            if !self.done {
                if effective_current > 0 {
                    replace_range_nth_char_ascii(&mut self.bar, effective_current, self.character);
                }
                replace_nth_char_ascii(&mut self.bar, effective_current, self.head);
            } else {
                replace_range_nth_char_ascii(&mut self.bar, self.ncols - 1, self.character);
                replace_nth_char_ascii(&mut self.bar, self.ncols - 1, self.character);
            }
        }
        if !self.done {
            print!(
                "\r{}",
                format_bar_push(
                    &self.message,
                    &self.bar,
                    self.current,
                    self.capacity,
                    &self.description
                )
            );
            io::stdout().flush().unwrap();
        } else {
            println!(
                "\r{}\n",
                format_bar_push(
                    &self.message,
                    &self.bar,
                    self.current,
                    self.capacity,
                    &self.description
                )
            );
        }
    }
}

impl ProgressBar {
    pub fn get_current(&self) -> usize {
        self.current
    }
    pub fn get_capacity(&self) -> usize {
        self.capacity
    }
    pub fn get_message(&self) -> &str {
        &self.message
    }
    pub fn get_description(&self) -> &str {
        &self.description
    }
    pub fn get_ncols(&self) -> usize {
        self.ncols
    }
    pub fn is_done(&self) -> bool {
        self.done
    }
    pub fn get_bar(&self) -> &str {
        &self.bar
    }
}
