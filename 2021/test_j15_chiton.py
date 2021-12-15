from inputj15 import sample_input


def parse_input(input):
    cave = []
    for row in input.split('\n'):
        cave.append(list(map(int, row)))
    return cave


def test_parse_input():
    assert parse_input(sample_input) == []


def is_min(actualMin, minx, miny, x, y, input, risk):
    if(y == len(input) or x == len(input[0]) or y < 0 or x < 0):
        return (actualMin, minx, miny)

    if(actualMin > risk + input[y][x]):
        return (risk + input[y][x], x, y)
    else:
        return (actualMin, minx, miny)


def find_lowset_risk(input, x, y, fromx, fromy, risk):
    minimum = risk
    if(y - 1 != fromy):
        minimum, minx, miny = is_min(minimum, x, y, x, y - 1, input, risk)
    if(x+1 != fromx):
        minimum, minx, miny = is_min(minimum, minx, miny, x + 1, y, input, risk)
    if(y+1 != fromy):
        minimum, minx, miny = is_min(minimum, minx, miny, x, y + 1, input, risk)
    if(x-1 != fromx):
        minimum, minx, miny = is_min(minimum, minx, miny, x - 1, y, input, risk)
    if(minx == 0 and miny == 0):
        return minimum
    else:
        return find_lowset_risk(input, minx, miny, x, y, risk)


def test_find_lowest_risk():
    x = len(sample_input[0]) - 1
    y = len(sample_input) - 1
    assert find_lowset_risk(sample_input, x, y, 0, 0, sample_input[y][x]) == 40
