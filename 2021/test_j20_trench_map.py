from inputj20 import sample_algo, sample_input, input, algo
from j20_trench_map import enhance, read_input, read_index, enhance_x


input_litted_border = """#####
#...#
##..#
#.#.#
#####"""


def test_enhance():
    assert len(enhance(enhance(read_input(sample_input), sample_algo), sample_algo)) == 35


def test_input():
    assert len(enhance(enhance(read_input(input), algo), algo)) == 5339


def xtest_50_iterations():
    assert len(enhance_x(50, read_input(sample_input), sample_algo)) == 3351
    assert len(enhance_x(50, read_input(input), algo)) == 18395


def test_read_index():
    assert read_index(2, 2, read_input(input_litted_border), 0, 0, 4, 4, False) == 34
    assert read_index(0, 0, read_input(input_litted_border), 0, 0, 4, 4, True) == int(
        '110111111', 2)
    assert read_index(0, 0, read_input(input_litted_border), 0, 0, 4, 4, False) == int(
        '010011000', 2)


def test_read_input():
    assert read_input(".#.\n#..") == set([(0, 0), (1, 1)])
