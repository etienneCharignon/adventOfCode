from j18_snailfish import sum_number, explode, reduce, split, sum_list, magnitude
from inputj18 import sample_input, input


def test_explod():
    assert explode(3, [4, 3]) == (False, [0, 0], [4, 3])
    assert explode(3, [[9, 8], 1]) == (True, [9, 0], [0, 9])
    assert explode(3, [4, [3, 2]]) == (True, [0, 2], [7, 0])
    assert explode(3, [[9, 8], [3, 2]]) == (True, [9, 0], [0, [11, 2]])
    assert explode(3, [[9, 8], [3, 2]]) == (True, [9, 0], [0, [11, 2]])


def test_explod_level2():
    assert explode(2, [4, 3]) == (False, [0, 0], [4, 3])
    assert explode(2, [[1, 2], [4, 3]]) == (False, [0, 0], [[1, 2], [4, 3]])
    assert explode(2, [[[9, 8], 1], 2]) == (True, [9, 0], [[0, 9], 2])
    assert explode(2, [3, [4, [3, 2]]]) == (True, [0, 2], [3, [7, 0]])
    assert explode(2, [[4, [3, 2]], 3]) == (True, [0, 0], [[7, 0], 5])
    assert explode(2, [3, [[3, 2], 4]]) == (True, [0, 0], [6, [0, 6]])
    assert explode(2, [[1, 3], [[3, 2], 4]]) == (True, [0, 0], [[1, 6], [0, 6]])
    assert explode(0, [[[[1, 2], 2], 2], 2])[0] is False
    assert explode(0, [[[[[5, 5], 2], 2], 2], 2])[0] is True
    assert explode(0, [[[[4, 0], [5, 4]], [[7, 7], [0, 13]]], [10, [[11, 9], [11, 0]]]]) == (
        False, [0, 0], [[[[4, 0], [5, 4]], [[7, 7], [0, 13]]], [10, [[11, 9], [11, 0]]]]
    )
    assert explode(0, [[[[4, 0], [5, 4]], [[7, 7], [0, [6, 7]]]], [10, [[11, 9], [11, 0]]]]) == (
        True, [0, 0], [[[[4, 0], [5, 4]], [[7, 7], [6, 0]]], [17, [[11, 9], [11, 0]]]]
    )


def test_split():
    assert split(9) == (False, 9)
    assert split(15) == (True, [7, 8])
    assert split([15, 3]) == (True, [[7, 8], 3])
    assert split([2, 10]) == (True, [2, [5, 5]])
    assert split([2, [11, 10]]) == (True, [2, [[5, 6], 10]])
    assert split([10, [11, 10]]) == (True, [[5, 5], [11, 10]])
    assert split([[[[10, 2], 2], 2], 2]) == (True, [[[[[5, 5], 2], 2], 2], 2])


def test_sum():
    assert sum_number([1, 1], [2, 2]) == [[1, 1], [2, 2]]
    assert sum_number([[[[4, 3], 4], 4], [7, [[8, 4], 9]]], [1, 1]) == [
        [[[0, 7], 4], [[7, 8], [6, 0]]], [8, 1]
    ]


def test_magnitude():
    assert magnitude(9) == 9
    assert magnitude([9, 1]) == 29
    assert magnitude([[1, 2], [[3, 4], 5]]) == 143
    assert magnitude([[[[8, 7], [7, 7]], [[8, 6], [7, 7]]], [[[0, 7], [6, 6]], [8, 7]]]) == 3488
    assert magnitude(sum_list(input)) == 4116


def test_reduce():
    assert reduce([[[[[9, 8], 1], 2], 3], 4]) == [[[[0, 9], 2], 3], 4]
    assert reduce([[6, [5, [4, [3, 2]]]], 1]) == [[6, [5, [7, 0]]], 3]
    assert reduce([[3, [2, [1, [7, 3]]]], [6, [5, [4, [3, 2]]]]]) == [
        [3, [2, [8, 0]]], [9, [5, [7, 0]]]
    ]


def test_sum_list():
    assert sum_list([[1, 1], [2, 2], [3, 3], [4, 4]]) == [
        [[[1, 1], [2, 2]], [3, 3]], [4, 4]
    ]
    assert sum_list([[1, 1], [2, 2], [3, 3], [4, 4], [5, 5]]) == [
        [[[3, 0], [5, 3]], [4, 4]], [5, 5]
    ]
    assert sum_list(sample_input) == [[[[8, 7], [7, 7]], [[8, 6], [7, 7]]], [[[0, 7], [6, 6]], [8, 7]]]
