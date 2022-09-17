use yapb::progress::ProgressBar;

pub fn main() {
    let mut pb = ProgressBar::new(100, None, None, None);
    for _ in 0..100 {
        pb.update(None, None);
        pb.render();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
