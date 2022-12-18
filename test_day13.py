from pathlib import Path

from unittest import TestCase

from day13 import parse_input, part_1, is_ordered, part_2

TEST_INPUT = """
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
"""


class TestDay13(TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.test_input = parse_input(TEST_INPUT)
        cls.real_input = parse_input(Path("input/day13.txt").read_text())

    def test_is_ordered(self) -> None:
        results = [is_ordered(a, b) for (a, b) in self.test_input]

        assert results == [True, True, False, True, False, True, False, False]

    def test_p1(self) -> None:
        for name, values, expected in (
            ("test_input", self.test_input, 13),
            ("real_input", self.real_input, 6076),
        ):
            with self.subTest(name):
                self.assertEqual(part_1(values), expected)

    def test_p2(self) -> None:
        for name, values, expected in (
            ("test_input", self.test_input, 140),
            ("real_input", self.real_input, 24805),
        ):
            with self.subTest(name):
                self.assertEqual(part_2(values), expected)
