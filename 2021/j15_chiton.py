from inputj15 import input


def parse_input(input):
    cave = []
    empty_grid = []
    for row in input.split('\n'):
        cave.append(list(map(int, row)))
        empty_grid.append([' ' for i in range(len(row))])
    return cave, empty_grid


def find_first_streat_path(input):
    x, y = 0, 0
    risk = input[y][x]
    while x < len(input[0])-1 and y < len(input)-1:
        y += 1
        risk += input[y][x]
        x += 1
        risk += input[y][x]
    return risk


def print_it(field):
    for row in field:
        print(' '.join(map(str, row)))


def print_visited(visited, input):
    input, screen = parse_input(input)
    for x, y in visited:
        screen[y][x] = input[y][x]
    print_it(screen)


def hide(input, path, x, y, known_min):
    if(y + 1 < len(input)):
        new_path = path + [input[y + 1][x]]
        if(sum(new_path) < known_min):
            return find_path(input, new_path, x, y + 1, known_min)
    if(x + 1 < len(input[0])):
        new_path = path + [input[y][x + 1]]
        if(sum(new_path) < known_min):
            return find_path(input, new_path, x + 1, y, known_min)


def find_path(input, visited, x, y, known_min, risk):
    visited.append((x, y))
    risk += input[y][x]
    # print(f"{x}, {y} : {risk}")
    # print_visited(visited, sample_input)
    if(risk >= known_min or len(visited) > 199):
        return None
    if(x + 1 == len(input[0]) and y + 1 == len(input)):
        return visited, risk

    to_visit = []
    if(x + 1 < len(input[0]) and (x + 1, y) not in visited):
        to_visit.append((input[y][x + 1], x + 1, y))
    if(y + 1 < len(input) and (x, y + 1) not in visited):
        to_visit.append((input[y + 1][x], x, y + 1))
    # if(x - 1 > 0 and (x - 1, y) not in visited):
    #     to_visit.append((input[y][x - 1], x - 1, y))
    # if(y - 1 > 0 and (x, y - 1) not in visited):
    #     to_visit.append((input[y - 1][x], x, y - 1))

    to_visit = sorted(to_visit, key=lambda direction: direction[0])
    for direction_to_visit in to_visit:
        r, x, y = direction_to_visit
        result = find_path(input, list(visited), x, y, known_min, risk)
        if(result is not None):
            return result
    return None


def compute_lower_risk(input_str):
    input, screen = parse_input(input_str)

    min_risk = find_first_streat_path(input)
    # min_risk = 100
    previous_risk = 0
    print(min_risk)
    while(previous_risk != min_risk):
        previous_risk = min_risk
        result = find_path(input, [], 0, 0, previous_risk, 0)
        if(result is None):
            return previous_risk
        visited, min_risk = result
        # print_visited(visited, input_str)
        print(f"{min_risk}, {len(visited)}")
    return min_risk


print(compute_lower_risk(input))
