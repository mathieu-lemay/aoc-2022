import json
import time
from typing import Optional


def debug_print(*args, **kwargs):
    # print(*args, **kwargs)
    pass


def parse_input(input_: str) -> list[tuple[list, list]]:
    values = list(
        map(json.loads, filter(lambda s: s.strip() != "", input_.split("\n")))
    )

    return [(values[i], values[i + 1]) for i in range(0, len(values), 2)]


def is_ordered(v1, v2, lvl=0) -> Optional[bool]:
    indent = " " * (lvl * 2)
    debug_print(f"{indent}- Compare {v1} vs {v2}")
    if isinstance(v1, int) and isinstance(v2, int):
        if v1 == v2:
            return None

        if v2 < v1:
            debug_print(
                f"{indent}   - Right side is smaller, so inputs are _not_ in the right order"
            )
            return False

        debug_print(
            f"{indent}   - Left side is smaller, so inputs are _in the right order_"
        )
        return True

    if isinstance(v1, list) and isinstance(v2, list):
        for idx, (a, b) in enumerate(zip(v1, v2)):
            if (res := is_ordered(a, b, lvl + 1)) is not None:
                return res

        if len(v1) == len(v2):
            return None

        if len(v2) < len(v1):
            debug_print(
                f"{indent}   - Right side ran out of items, so inputs are _not_ in the right order"
            )
            return False

        debug_print(
            f"{indent}   - Left side ran out of items, so inputs are _in the right order_"
        )
        return True

    if isinstance(v1, int):
        v1 = [v1]
        debug_print(
            f"{indent}   - Mixed types; convert left to {v1} and retry comparison"
        )

    if isinstance(v2, int):
        v2 = [v2]
        debug_print(
            f"{indent}   - Mixed types; convert right to {v2} and retry comparison"
        )

    return is_ordered(v1, v2, lvl + 1)


def part_1(values: list[tuple[list, list]]) -> int:
    return sum(i + 1 for i, v in enumerate(values) if is_ordered(*v) is not False)


def part_2(values: list[tuple[list, list]]) -> int:
    div2 = [[2]]
    div6 = [[6]]
    count2 = 0
    count6 = 0

    for v in values:
        for w in v:
            if is_ordered(w, div2):
                count2 += 1
                count6 += 1
            elif is_ordered(w, div6):
                count6 += 1

    x = count2 + 1
    y = count6 + 2

    return x * y


def main():
    with open("input/day13.txt") as f:
        values = parse_input(f.read())

    t = time.perf_counter()
    print("Part 1:", part_1(values))
    print("Part 2:", part_2(values))
    t = time.perf_counter() - t
    print(f"Processed in {t*1000:.3f}ms")


if __name__ == "__main__":
    main()
