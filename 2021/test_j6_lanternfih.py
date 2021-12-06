from j6_lanternfish import next_generation, init_world


def test_next_generation_no_new_fish():
    assert next_generation(init_world('3,2')) == {0: 0, 1: 1, 2: 1, 3: 0, 4: 0, 5: 0, 6: 0, 7: 0, 8: 0}


def test_next_generation_new_fish():
    assert next_generation(init_world('0')) == {0: 0, 1: 0, 2: 0, 3: 0, 4: 0, 5: 0, 6: 1, 7: 0, 8: 1}
    assert next_generation(init_world('0,7')) == {0: 0, 1: 0, 2: 0, 3: 0, 4: 0, 5: 0, 6: 2, 7: 0, 8: 1}


def test_init_world():
    assert init_world('1,2') == {0: 0, 1: 1, 2: 1, 3: 0, 4: 0, 5: 0, 6: 0, 7: 0, 8: 0}
