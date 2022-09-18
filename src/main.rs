use yapb::progress::ProgressBar;

pub fn main() {
    // let term = Term::stdout();
    // term.write_line("Hello World!").unwrap();
    // thread::sleep(Duration::from_millis(2000));
    // term.clear_last_lines(1).unwrap();
    const N: usize = 100_000;
    let mut pb = ProgressBar::new(N, None, None, None);
    for _ in 0..N {
        pb.update(None, None);
        pb.render();
    }
}
