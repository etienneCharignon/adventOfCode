def init_world(input):
    world = {0: 0, 1: 0, 2: 0, 3: 0, 4: 0, 5: 0, 6: 0, 7: 0, 8: 0}

    def add(point, diagram):
        if(point in diagram):
            diagram[point] += 1
        else:
            diagram[point] = 1

    inputs = input.split(',')
    for fish in inputs:
        add(int(fish), world)

    return world


def next_generation(world):
    new_fish = []

    def decrease_day(fish):
        if(fish == 0):
            new_fish.append(8)
            return 6
        return fish - 1

    result = {}
    result[0] = world[1]
    result[1] = world[2]
    result[2] = world[3]
    result[3] = world[4]
    result[4] = world[5]
    result[5] = world[6]
    result[6] = world[0] + world[7]
    result[7] = world[8]
    result[8] = world[0]

    return result
