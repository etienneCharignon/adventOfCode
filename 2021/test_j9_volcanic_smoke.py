from samplej9 import input
from inputj9 import rinput
from j9_volcanic_smoke import find_lower_point, parse_input


def test_find_lower_point():
    assert find_lower_point([[1, 0, 1]]) == 1
    assert find_lower_point([[1], [0], [1]]) == 1


def test_parse_input():
    world = parse_input(input)
    assert len(world) == 5
    assert len(world[0]) == 10


def test_count_lower_point():
    assert find_lower_point(parse_input(input)) == 15
    assert find_lower_point(parse_input(rinput)) == 550
