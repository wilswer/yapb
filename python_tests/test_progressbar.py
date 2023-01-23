import yapb


def test_progressbar() -> None:
    N = 100
    pb = yapb.ProgressBar(N)
    for _ in range(N):
        pb.update()
        pb.render()
