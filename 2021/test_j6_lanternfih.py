from j6_lanternfish import next_generation


def test_next_generation_no_new_fish():
    assert next_generation([3, 2]) == [2, 1]


def test_next_generation_one_fish():
    assert next_generation([0]) == [6, 8]
