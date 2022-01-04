from inputj22 import example_input
from j12_cuboid import LightedCuboid, intersection, is_empty


def read_coordinate(string):
    axe, coordinate = string.split('=')
    return tuple(map(int, coordinate.split('..')))


def read_line(line):
    on, coordinates = line.split(' ')
    on = on == 'on'
    x, y, z = coordinates.split(',')
    return (on, read_coordinate(x), read_coordinate(y), read_coordinate(z))


def test_read_input():
    assert read_line('on x=1..2,y=3..4,z=5..6') == (True, (1, 2), (3, 4), (5, 6))


def count_cubes(cuboids):
    count = 0
    for cuboid in cuboids:
        count += LightedCuboid(cuboid).volume()
    return count


def test_is_empty():
    assert is_empty(((1, 0), (1, 2), (1, 2)))
    assert is_empty(((1, 2), (1, 0), (1, 2)))
    assert is_empty(((1, 2), (1, 2), (1, 0)))
    assert not is_empty(((1, 2), (1, 2), (1, 1)))


def generate_world(lines):
    lighted = []
    to_remove = []
    for line in lines:
        on, x, y, z = read_line(line)
        if(on):
            for cuboid in lighted:
                intersection_cuboid = intersection((x, y, z), cuboid)
                if(not is_empty(intersection_cuboid)):
                    to_remove.append(intersection_cuboid)
            lighted.append((x, y, z))
        else:
            for cuboid in lighted:
                intersection_cuboid = intersection((x, y, z), cuboid)
                if(not is_empty(intersection_cuboid)):
                    to_remove.append(intersection_cuboid)
                    break

    return (lighted, to_remove)


def test_generate_cuboid():
    assert generate_world(['on x=10..12,y=10..12,z=10..12']) == ([((10, 12), (10, 12), (10, 12))], [])
    assert generate_world(['on x=10..12,y=10..12,z=10..12', 'on x=9..13,y=10..13,z=10..12']) == ([
        ((10, 12), (10, 12), (10, 12)),
        ((9, 13), (10, 13), (10, 12))
    ],
        [((10, 12), (10, 12), (10, 12))]
    )
    assert generate_world(['on x=10..12,y=10..12,z=10..12',
                           'on x=11..13,y=11..13,z=11..13',
                           'on x=10..10,y=10..10,z=10..10']) == ([
                               ((10, 12), (10, 12), (10, 12)),
                               ((11, 13), (11, 13), (11, 13)),
                               ((10, 10), (10, 10), (10, 10))
                           ], [
                               ((11, 12), (11, 12), (11, 12)),
                               ((10, 10), (10, 10), (10, 10))
                           ])


def test_generate_cuboid_turn_off_ligths():
    assert generate_world(['on x=10..12,y=10..12,z=10..12',
                           'on x=11..13,y=11..13,z=11..13',
                           'off x=9..11,y=9..11,z=9..11']) == ([
                               ((10, 12), (10, 12), (10, 12)),
                               ((11, 13), (11, 13), (11, 13)),
                           ], [
                               ((11, 12), (11, 12), (11, 12)),
                               ((10, 11), (10, 11), (10, 11)),
                           ])
    assert generate_world(['off x=9..11,y=9..11,z=9..11']) == ([], [])


def solve(input):
    world = generate_world(input.split('\n'))
    return count_cubes(world[0]) - count_cubes(world[1])


def xtest_solve():
    assert solve(example_input) == 39


def test_cuboid():
    cuboid = LightedCuboid(((10, 12), (10, 12), (10, 12)))
    assert cuboid.lighted() == 3 * 3 * 3
    cuboid.remove(((12, 13), (10, 13), (10, 12)))
    assert cuboid.lighted() == 19
