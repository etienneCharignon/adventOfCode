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


def is_lower(point, world):
    line, col = point
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
    return isLower


def find_basins(world):
    basins = {}
    for line in range(0, len(world)):
        for col in range(0, len(world[line])):
            if(is_lower((line, col), world)):
                basins[(line, col)] = 1
    return basins


def find_lowest(world, point):
    line, col = point
    current = world[line][col]
    if(col > 0):
        left = world[line][col - 1]
        if(left < current):
            return find_lowest(world, (line, col - 1))
    if(col < len(world[line]) - 1):
        right = world[line][col + 1]
        if(right < current):
            return find_lowest(world, (line, col + 1))
    if(line > 0):
        up = world[line - 1][col]
        if(up < current):
            return find_lowest(world, (line - 1, col))
    if(line < len(world) - 1):
        down = world[line + 1][col]
        if(down < current):
            return find_lowest(world, (line + 1, col))
    return point


def find_wich_basin(world, current, basins):
    l, c = current
    if(is_lower(current, world) or world[l][c] == 9):
        return basins
    basins[find_lowest(world, current)] += 1
    return basins
