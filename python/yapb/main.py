import time
from tqdm import tqdm
from yapb import ProgressBar


def base_time(n_its: int, wait_time: float) -> None:
    start = time.time()
    for _ in range(n_its):
        if wait_time > 0:
            time.sleep(wait_time)
    end = time.time()
    print(end - start)


def yapb_time(n_its: int, wait_time: float) -> None:
    pb = ProgressBar(n_its, "Loading", "Please wait")
    start = time.time()
    for _ in range(n_its):
        pb.update(description=f"{_} of {n_its}")
        pb.render()
        if wait_time > 0:
            time.sleep(wait_time)
    end = time.time()
    print(end - start)


def tqdm_time(n_its: int, wait_time: float) -> None:
    pb = tqdm(total=n_its, desc="Loading", position=0, leave=True)
    start = time.time()
    for _ in range(n_its):
        pb.update()
        pb.set_description(f"{_} of {n_its}")
        if wait_time > 0:
            time.sleep(wait_time)
    end = time.time()
    print(end - start)


def main() -> None:
    N = 100_000
    WAIT_TIME = 0.0
    base_time(N, WAIT_TIME)
    yapb_time(N, WAIT_TIME)
    tqdm_time(N, WAIT_TIME)


if __name__ == "__main__":
    main()
