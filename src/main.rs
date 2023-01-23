use yapb::progress::ProgressBar;

pub fn main() {
    const N: usize = 100_000;
    let mut pb = ProgressBar::new(N, None, None, None);
    for _ in 0..N {
        pb.update(None, None);
        pb.render();
    }
}
