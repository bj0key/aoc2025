import numpy as np
from time import time

def get_input() -> np.ndarray:
    with open("day4/input") as f:
        data = f.read().strip()

    lists = [[1 if c == "@" else 0 for c in line] for line in data.splitlines()]
    return np.array(lists, dtype=np.uint8)


def reachable_mask(arr: np.ndarray) -> np.ndarray:
    mask = np.zeros(arr.shape, dtype=np.uint8)

    rows, cols = arr.shape
    for r in range(rows):
        for c in range(cols):
            if arr[r, c] != 1:
                continue

            min_r = max(0, r - 1)
            max_r = min(rows, r + 2)
            min_c = max(0, c - 1)
            max_c = min(cols, c + 2)

            neighbors = arr[min_r:max_r, min_c:max_c].sum()
            if neighbors < 5:
                mask[r, c] = 1

    return mask


def part1(arr: np.ndarray) -> int:
    return reachable_mask(arr).sum(dtype=np.uint64)


def part2(arr: np.ndarray) -> int:
    reachable = np.zeros(arr.shape, dtype=np.uint8)
    reached = -1
    while reached != 0:
        new_reachable = reachable_mask(arr)
        reachable |= new_reachable
        arr ^= new_reachable
        reached = new_reachable.sum(dtype=np.uint64)

    return reachable.sum(dtype=np.uint64)


def main():
    start = time()
    np.set_printoptions(threshold=2**30)
    inp = get_input()

    p1 = part1(inp)
    p2 = part2(inp)
    end = time()
    print(f"Part 1: {p1}")
    print(f"Part 2: {p2}")
    print(f"Done in {end-start}s")


if __name__ == "__main__":
    main()
