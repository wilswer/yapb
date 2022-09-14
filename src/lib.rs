use pyo3::prelude::*;
use std::io::Write;

macro_rules! progressbar_fmt_str {
    () => {
        "\r{} [{}] {}/{} {}"
    };
}

/// A Python module implemented in Rust.
#[pymodule]
fn yapb(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ProgressBar>()?;
    m.add_class::<FastProgressBar>()?;
    // m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
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
        } else {
            println!(
                "\r{} [{}] {}/{} {}",
                self.message, bar, self.current, self.length, self.description
            );
        }
    }
}

#[pyclass]
pub struct FastProgressBar {
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
impl FastProgressBar {
    #[new]
    pub fn new(length: usize, message: String, description: String) -> FastProgressBar {
        FastProgressBar {
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
            let bar_str = format!(
                progressbar_fmt_str!(),
                self.message, bar, self.current, self.length, self.description
            );
            let stdout = std::io::stdout();
            let lock = stdout.lock();
            let mut buf = std::io::BufWriter::with_capacity(8, lock);
            let _ = buf.write(bar_str.as_bytes()).unwrap();
        } else {
            println!(
                "\r{} [{}] {}/{} {}",
                self.message, bar, self.current, self.length, self.description
            );
        }
    }
}
