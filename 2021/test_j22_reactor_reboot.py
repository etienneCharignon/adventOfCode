# from inputj22 import example_input

space = [[[False]*50]*50]*50


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


def length(axe):
    return axe[1] - axe[0] + 1


def volume(cuboid):
    x, y, z = cuboid
    return length(x) * length(y) * length(z)


def count_cubes(cuboids):
    count = 0
    for cuboid in cuboids:
        count += volume(cuboid)
    return count


def computes_new_cuboids(x, y, z, world):
    new_cuboids = []
    for cuboid in world:
        if(x[1] > cuboid[0][1]):
            new_x = (cuboid[0][1] + 1, x[1])
            new_cuboids.append((new_x, y, z))
        if(x[0] < cuboid[0][0]):
            new_x = (x[0], cuboid[0][0] - 1)
            new_cuboids.append((new_x, y, z))
        if(y[1] > cuboid[1][1]):
            new_y = (cuboid[1][1] + 1, y[1])
            new_cuboids.append((cuboid[0], new_y, z))

    if(len(new_cuboids) == 0):
        new_cuboids.append((x, y, z))

    return new_cuboids


def generate_cuboid(lines):
    world = []
    for line in lines:
        on, x, y, z = read_line(line)
        new_cuboids = computes_new_cuboids(x, y, z, world)
        world += new_cuboids

    return world


def test_generate_cuboid():
    assert generate_cuboid(['on x=10..12,y=10..12,z=10..12']) == [((10, 12), (10, 12), (10, 12))]
    assert generate_cuboid(['on x=10..12,y=10..12,z=10..12', 'on x=9..13,y=10..13,z=10..12']) == [
        ((10, 12), (10, 12), (10, 12)),
        ((13, 13), (10, 13), (10, 12)),
        ((9, 9), (10, 13), (10, 12)),
        ((10, 12), (13, 13), (10, 12))
    ]


def xtest_compute_new_cuboids():
    assert computes_new_cuboids((15, 16), (10, 12), (10, 12), [((10, 12), (10, 12), (10, 12))]) == [
        ((15, 16), (10, 12), (10, 12))
    ]


def set_space(cuboid):
    None


def test_set_space():
    assert set_space(((10, 12), (10, 12), (10, 12)))[10][10][10]


def test_count_cubes():
    assert count_cubes([((10, 12), (10, 12), (10, 12))]) == 27
