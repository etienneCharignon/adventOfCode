from inputj15 import sample_input  # , input
from j15_chiton import compute_lower_risk_slow, compute_lower_risk, find_path, parse_input


def test_parse_input():
    assert parse_input(sample_input)[0][0][0] == 1


def test_find_path():
    assert find_path([[1, 1, 1]], [], 0, 0, 10, 0) == (
        [(0, 0), (1, 0), (2, 0)], 3
    )
    assert find_path([[1], [1], [1]], [], 0, 0, 10, 0) == (
        [(0, 0), (0, 1), (0, 2)], 3
    )
    assert find_path([[1, 2], [1, 3]], [], 0, 0, 10, 0) == (
        [(0, 0), (0, 1), (1, 1)], 5
    )
    assert find_path([[1, 2, 3], [4, 5, 6]], [], 0, 0, 15, 0) == (
        [(0, 0), (1, 0), (2, 0), (2, 1)], 12
    )
    assert find_path([[3, 4, 5], [1, 2, 0]], [], 0, 0, 7, 0) == (
        [(0, 0), (0, 1), (1, 1), (2, 1)], 6
    )
    assert find_path([[3, 2, 1], [1, 3, 0]], [], 0, 0, 7, 0) == (
        [(0, 0), (1, 0), (2, 0), (2, 1)], 6
    )


def test_find_lowest_risk():
    assert compute_lower_risk_slow(sample_input) == 41
    assert compute_lower_risk(sample_input) == 41
