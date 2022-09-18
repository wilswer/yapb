use yapb::progress::ProgressBar;

#[test]
pub fn test_current() {
    let mut pb = ProgressBar::new(100, None, None, None);
    assert_eq!(pb.get_current(), 0);
    pb.update(None, None);
    assert_eq!(pb.get_current(), 1);
}

#[test]
pub fn test_capacity() {
    let mut pb = ProgressBar::new(100, None, None, None);
    assert_eq!(pb.get_capacity(), 100);
    pb.update(None, None);
    pb.render();
    assert_eq!(pb.get_capacity(), 100);
}

#[test]
pub fn test_ncols() {
    let mut pb = ProgressBar::new(100, None, None, Some(50));
    assert_eq!(pb.get_ncols(), 50);
    pb.update(None, None);
    pb.render();
    assert_eq!(pb.get_ncols(), 50);
}

#[test]
pub fn test_empty_message() {
    let mut pb = ProgressBar::new(100, None, None, None);
    assert_eq!(pb.get_message(), "");
    pb.update(None, None);
    pb.render();
    assert_eq!(pb.get_message(), "");
}

#[test]
pub fn test_message() {
    let mut pb = ProgressBar::new(100, Some("Hello".to_string()), None, None);
    assert_eq!(pb.get_message(), "Hello");
    pb.update(Some("Hello".to_string()), None);
    pb.render();
    assert_eq!(pb.get_message(), "Hello");
}

#[test]
pub fn test_empty_description() {
    let mut pb = ProgressBar::new(100, None, None, None);
    assert_eq!(pb.get_description(), "");
    pb.update(None, None);
    pb.render();
    assert_eq!(pb.get_description(), "");
}

#[test]
pub fn test_description() {
    let mut pb = ProgressBar::new(100, None, Some("Hello".to_string()), None);
    assert_eq!(pb.get_description(), "Hello");
    pb.update(None, Some("Hello".to_string()));
    pb.render();
    assert_eq!(pb.get_description(), "Hello");
}

#[test]
pub fn test_done() {
    let mut pb = ProgressBar::new(100, None, None, None);
    assert_eq!(pb.is_done(), false);
    for i in 0..100 {
        pb.update(None, None);
        pb.render();
        if i < 99 {
            assert_eq!(pb.is_done(), false);
        } else {
            assert_eq!(pb.is_done(), true);
        }
    }
    assert_eq!(pb.is_done(), true);
}

#[test]
pub fn test_bar_high_capacity() {
    const CAPACITY: usize = 1_000;
    const NCOLS: usize = 88;
    let mut pb = ProgressBar::new(CAPACITY, None, None, Some(NCOLS));
    assert_eq!(pb.get_bar(), format!("{}", " ".repeat(NCOLS)));
    for _ in 0..CAPACITY {
        pb.update(None, None);
        pb.render();
        assert_eq!(pb.get_bar().len(), NCOLS);
        let effective_current = ((pb.get_current() as f64 / pb.get_capacity() as f64)
            * (pb.get_ncols() as f64)) as usize;
        assert_eq!(
            pb.get_bar(),
            if !pb.is_done() {
                format!(
                    "{}{}{}",
                    "=".repeat(effective_current),
                    ">".repeat(1),
                    " ".repeat(NCOLS - effective_current - 1)
                )
            } else {
                format!("{}", "=".repeat(NCOLS))
            }
        );
    }
}

#[test]
pub fn test_bar_low_capacity() {
    const CAPACITY: usize = 10;
    const NCOLS: usize = 77;
    let mut pb = ProgressBar::new(CAPACITY, None, None, Some(NCOLS));
    assert_eq!(pb.get_bar(), format!("{}", " ".repeat(NCOLS)));
    for _ in 0..CAPACITY {
        pb.update(None, None);
        pb.render();
        assert_eq!(pb.get_bar().len(), NCOLS);
        let effective_current = ((pb.get_current() as f64 / pb.get_capacity() as f64)
            * (pb.get_ncols() as f64)) as usize;
        assert_eq!(
            pb.get_bar(),
            if !pb.is_done() {
                format!(
                    "{}{}{}",
                    "=".repeat(effective_current),
                    ">".repeat(1),
                    " ".repeat(NCOLS - effective_current - 1)
                )
            } else {
                format!("{}", "=".repeat(NCOLS))
            }
        );
    }
}

#[test]
pub fn test_log() {
    let mut pb = ProgressBar::new(100, None, None, None);
    assert!(pb._get_logs().is_empty());
    pb.update(None, None);
    pb.render();
    assert!(pb._get_logs().is_empty());

    pb.log("Hello".to_string());
    assert_eq!(pb._get_logs().len(), 1);
    assert_eq!(pb._get_logs()[0].0, 1);
    assert_eq!(pb._get_logs()[0].1, "Hello".to_string());
    pb.log("Hello".to_string());
    assert_eq!(pb._get_logs().len(), 1);
    assert_eq!(pb._get_logs()[0].0, 1);
    assert_eq!(pb._get_logs()[0].1, "Hello".to_string());
    pb.reset();
    for i in 0..100 {
        pb.log(i.to_string());
        assert_eq!(pb._get_logs().len(), i + 1);
        assert_eq!(pb._get_logs()[i].0, i);
        assert_eq!(pb._get_logs()[i].1, i.to_string());
        pb.update(None, None);
    }
}

#[test]
pub fn test_refresh() {
    let mut pb = ProgressBar::new(100, None, None, None);
    pb.update(None, None);
    assert_eq!(pb.get_current(), 1);
    pb.refresh(None, None);
    assert_eq!(pb.get_current(), 2);
    pb.refresh(None, None);
    assert_eq!(pb.get_current(), 3);
    pb.refresh(None, None);
    assert_eq!(pb.get_current(), 4);
    pb.refresh(None, None);
    assert_eq!(pb.get_current(), 5);
    pb.refresh(None, None);
    assert_eq!(pb.get_current(), 6);
    pb.refresh(None, None);
    assert_eq!(pb.get_current(), 7);
    pb.refresh(None, None);
    assert_eq!(pb.get_current(), 8);
    pb.refresh(None, None);
    assert_eq!(pb.get_current(), 9);
    pb.refresh(None, None);
    assert_eq!(pb.get_current(), 10);
    pb.refresh(None, None);
    assert_eq!(pb.get_current(), 11);
    pb.refresh(None, None);
    assert_eq!(pb.get_current(), 12);
    pb.refresh(None, None);
    assert_eq!(pb.get_current(), 13);
    pb.refresh(None, None);
    assert_eq!(pb.get_current(), 14);
    pb.refresh(None, None);
    assert_eq!(pb.get_current(), 15);
    pb.refresh(None, None);
    assert_eq!(pb.get_current(), 16);
    pb.refresh(None, None);
    assert_eq!(pb.get_current(), 17);
    pb.refresh(None, None);
    assert_eq!(pb.get_current(), 18);
    pb.refresh(None, None);
    assert_eq!(pb.get_current(), 19);
    pb.refresh(None, None);
    assert_eq!(pb.get_current(), 20);
    pb.refresh(None, None);
    assert_eq!(pb.get_current(), 21);
    pb.refresh(None, None);
    assert_eq!(pb.get_current(), 22);
    pb.refresh(None, None);
    assert_eq!(pb.get_current(), 23);
    pb.refresh(None, None);
    assert_eq!(pb.get_current(), 24);
    pb.refresh(None, None);
    assert_eq!(pb.get_current(), 25);
    pb.refresh(None, None);
    assert_eq!(pb.get_current(), 26);
    pb.refresh(None, None);
    assert_eq!(pb.get_current(), 27);
    pb.refresh(None, None);
    assert_eq!(pb.get_current(), 28);
    pb.refresh(None, None);
    assert_eq!(pb.get_current(), 29);
    pb.refresh(None, None);
    assert_eq!(pb.get_current(), 30);
    pb.refresh(None, None);
}
