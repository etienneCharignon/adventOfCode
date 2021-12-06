def init_world(input):
    world = {0: 0, 1: 0, 2: 0, 3: 0, 4: 0, 5: 0, 6: 0, 7: 0, 8: 0}

    def add(point, diagram):
        diagram[point] += 1

    inputs = input.split(',')
    for fish in inputs:
        world[int(fish)] += 1

    return world


def next_generation(world):
    new_world = {}
    new_world[0] = world[1]
    new_world[1] = world[2]
    new_world[2] = world[3]
    new_world[3] = world[4]
    new_world[4] = world[5]
    new_world[5] = world[6]
    new_world[6] = world[0] + world[7]
    new_world[7] = world[8]
    new_world[8] = world[0]

    return new_world
