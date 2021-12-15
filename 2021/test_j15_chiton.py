from inputj15 import sample_input, input


def parse_input(input):
    cave = []
    risks = []
    for row in input.split('\n'):
        cave.append(list(map(int, row)))
        risks.append([100 for i in range(len(row))])
    return cave, risks


def print_it(field):
    for row in field:
        print(' '.join(map(str, row)))


def test_parse_input():
    assert parse_input(sample_input)[0][0][0] == 1


def compute_risk(risks, input, x, y, risk):
    if(x < 0 or y < 0):
        return risks
    risks[y][x-1] = min(risk + input[y][x-1], risks[y][x-1])
    risks = compute_risk(risks, input, x - 1, y, risks[y][x-1])
    risks[y-1][x] = min(risk + input[y-1][x], risks[y-1][x])
    risks = compute_risk(risks, input, x, y - 1, risks[y-1][x])
    return risks


def compute_lower_risk(input):
    input, risks = parse_input(input)
    x = len(input[0]) - 1
    y = len(input) - 1
    print(x)
    print(y)
    assert False
    risks = compute_risk(risks, input, x, y, input[y][x])
    print_it(risks)
    return risks[0][0] - input[0][0]


def test_find_lowest_risk():
    assert compute_lower_risk(input) == 40
