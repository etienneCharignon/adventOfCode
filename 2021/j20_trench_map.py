def print_it(litted):
    minx = min(litted, key = lambda position: position[0])[0]
    miny = min(litted, key = lambda position: position[1])[1]
    maxx = max(litted, key = lambda position: position[0])[0]
    maxy = max(litted, key = lambda position: position[1])[1]
    screen = [None] * (maxy - min(0, miny) + 1)
    for row in range(0, maxy - min(0, miny) + 1):
        screen[row] = ['.'] * (maxx - min(0, minx) + 1)

    for x, y in litted:
        screen[y + abs(miny)][x + abs(minx)] = '#'

    screen.reverse()
    for row in screen:
        print(''.join(row))

def read_index(x, y, litted, minx, miny, maxx, maxy, infinit_litted):
    binary = ''
    for dy in range(1, -2, -1):
        for dx in range(-1, 2):
            coordinates = (x + dx, y + dy)
            if(coordinates[0] < minx or coordinates[0] > maxx or
               coordinates[1] < miny or coordinates[1] > maxy):
                digit = ('0', '1')[infinit_litted]
            else:
                digit = 0
                if(coordinates in litted):
                    digit = '1'
                else:
                    digit = '0'
            binary += digit
            # print(f"{coordinates} : {digit}")
    # print(binary)
    return int(binary, 2)


def enhance(input, algo):
    minx = min(input, key = lambda position: position[0])[0]
    miny = min(input, key = lambda position: position[1])[1]
    maxx = max(input, key = lambda position: position[0])[0]
    maxy = max(input, key = lambda position: position[1])[1]
    enhance_litted = set()
    count = 0
    for x in range(minx, maxx + 1):
        if((x, miny) in input):
            count += 1
    infinit_litted = (count == maxx - minx + 1)
    for x in range(minx - 10, maxx + 11):
        for y in range(miny - 10, maxy + 11):
            index = read_index(x, y, input, minx, miny, maxx, maxy, infinit_litted)
            if(algo[index] == '#'):
                enhance_litted.add((x, y))

    # print("-----------------------")
    # print(infinit_litted)
    # print_it(enhance_litted)
    return enhance_litted


def read_input(input):
    litted = set()
    rows = input.split('\n')
    rows.reverse()
    for y in range(0, len(rows)):
        for x in range(0, len(rows[y])):
            if(rows[y][x] == '#'):
                litted.add((x, y))
    return litted


def enhance_x(iteration, input, algo):
    for n in range(0, iteration):
        input = enhance(input, algo)
    return input
