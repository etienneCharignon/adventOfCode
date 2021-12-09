def parse_input(input):
    world = []
    for row in input.split('\n'):
        line = []
        world.append(line)
        for height in row:
            line.append(int(height))
    return world


def find_lower_point(world):
    low_point = 0
    for line in range(0, len(world)):
        for col in range(0, len(world[line])):
            current = world[line][col]
            isLower = True
            if(col > 0):
                isLower = current < world[line][col - 1]
            if(col < len(world[line]) - 1):
                isLower = isLower and current < world[line][col + 1]
            if(line > 0):
                isLower = isLower and current < world[line - 1][col]
            if(line < len(world) - 1):
                isLower = isLower and current < world[line + 1][col]
            if(isLower):
                low_point += current + 1
    return low_point
