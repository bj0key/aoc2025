def get_input() -> list[list[int]]:
    with open("day3/input") as f:
        data = f.read().strip()
    return [[int(i) for i in line] for line in data.splitlines()]


def next_max(bank: list[int], n: int) -> tuple[int, int]:
    return max(enumerate(bank[: len(bank) + 1 - n]), key=lambda x: x[1])


def biggest_battery(bank: list[int], length: int) -> int:
    total = 0
    for i in range(length, 0, -1):
        idx, next_digit = next_max(bank, i)
        total = 10 * total + next_digit
        bank = bank[idx + 1 :]
    return total


def main():
    banks = get_input()
    part1 = sum(biggest_battery(b, 2) for b in banks)
    print(f"Part 1: {part1}")
    part2 = sum(biggest_battery(b, 12) for b in banks)
    print(f"Part 1: {part2}")


if __name__ == "__main__":
    main()
