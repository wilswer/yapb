use pyo3::prelude::*;
use std::io::{self, Write};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn yapb(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ProgressBar>()?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
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
}

#[pymethods]
impl ProgressBar {
    #[new]
    pub fn new(length: usize, message: String, description: String) -> ProgressBar {
        ProgressBar {
            length,
            current: 0,
            message,
            description,
            character: '=',
            head: '>',
            done: false,
            max_length: 88,
        }
    }

    pub fn get_length(&self) -> usize {
        self.length
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

    pub fn render(&self) {
        let effective_current: usize =
            ((self.current as f64 / self.length as f64) * (self.max_length as f64)) as usize;
        let mut bar = String::new();
        if effective_current > 0 {
            bar.push_str(&self.character.to_string().repeat(effective_current - 1));
        }
        if !self.done {
            bar.push(self.head);
        } else {
            bar.push(self.character);
        }
        bar.push_str(&' '.to_string().repeat(self.max_length - effective_current));
        if !self.done {
            print!(
                "\r{} [{}] {}/{} {}",
                self.message, bar, self.current, self.length, self.description
            );
            io::stdout().flush().unwrap();
        } else {
            println!(
                "\r{} [{}] {}/{} {}",
                self.message, bar, self.current, self.length, self.description
            );
        }
    }
}
