from j5_hydrothermal_vent import direction


def test_direction():
    assert direction(1, 2) == 1
    assert direction(2, 1) == -1
    assert direction(3, 1) == -1
