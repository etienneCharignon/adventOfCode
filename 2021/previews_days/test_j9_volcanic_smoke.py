import numpy
from samplej9 import input
from inputj9 import rinput
from j9_volcanic_smoke import find_lower_point, parse_input, find_basins, find_wich_basin


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


def test_find_basins():
    assert find_basins(parse_input(input)) == {
        (0, 1): 1,
        (0, 9): 1,
        (2, 2): 1,
        (4, 6): 1
    }


def test_find_wich_basin():
    assert find_wich_basin([[1, 0, 1]], (0, 2), {(0, 1): 1}) == {(0, 1): 2}
    assert find_wich_basin([[1, 0, 9]], (0, 2), {(0, 1): 1}) == {(0, 1): 1}


def test_find_all_basins():
    world = parse_input(input)
    basins = find_basins(world)
    for line in range(0, len(world)):
        for col in range(0, len(world[line])):
            basins = find_wich_basin(world, (line, col), basins)
    assert basins == {(0, 1): 3, (0, 9): 9, (2, 2): 14, (4, 6): 9}

    basins_sizes = list(reversed(sorted(basins.values())))
    tree_biggest = [basins_sizes[index] for index in [0, 1, 2]]
    assert numpy.prod(tree_biggest) == 1134


def test_find_all_rbasins():
    world = parse_input(rinput)
    basins = find_basins(world)
    for line in range(0, len(world)):
        for col in range(0, len(world[line])):
            basins = find_wich_basin(world, (line, col), basins)

    basins_sizes = list(reversed(sorted(basins.values())))
    tree_biggest = [basins_sizes[index] for index in [0, 1, 2]]
    assert numpy.prod(tree_biggest) == 1100682
