import math


def magnitude(number):
    if(isinstance(number, list)):
        a, b = number
        return 3 * magnitude(a) + 2 * magnitude(b)
    return number


def sum_list(homework):
    the_sum = homework[0]
    for index in range(1, len(homework)):
        the_sum = sum_number(the_sum, homework[index])
    return the_sum


def sum_number(a, b):
    return reduce([a, b])


def propagate_explosion_b(explosion, pair):
    if(isinstance(pair, list)):
        a, b = pair
        return [a, propagate_explosion_b(explosion, b)]
    else:
        return pair + explosion


def propagate_explosion_a(explosion, pair):
    if(isinstance(pair, list)):
        a, b = pair
        return [propagate_explosion_a(explosion, a), b]
    else:
        return pair + explosion


def explode(depth, pair):
    a, b = pair

    if(depth < 3):
        if(isinstance(a, list)):
            (exploded, explosion, pair) = explode(depth + 1, a)
            if(exploded):
                b = propagate_explosion_a(explosion[1], b)
                return (exploded, [explosion[0], explosion[1] - explosion[1]], [pair, b])

        if(isinstance(b, list)):
            (exploded, explosion, pair) = explode(depth + 1, b)
            if(exploded):
                a = propagate_explosion_b(explosion[0], a)
                return (exploded, [explosion[0] - explosion[0], explosion[1]], [a, pair])
        return (False, [0, 0], [a, b])

    if(isinstance(a, list)):
        if(isinstance(b, list)):
            return (True, [a[0], 0], [0, [a[1] + b[0], b[1]]])
        else:
            return (True, [a[0], 0], [0, a[1] + b])
    elif(isinstance(b, list)):
        print(f"explosion : {b[1]}")
        return (True, [0, b[1]], [b[0] + a, 0])
    else:
        return False, [0, 0], pair


def split(pair):
    if(isinstance(pair, list)):
        a, b = pair
        splited, a = split(a)
        if(not splited):
            splited, b = split(b)
        return splited, [a, b]
    else:
        if(pair > 9):
            return True, [int(pair / 2), int(math.ceil(pair / 2))]
        return False, pair


def reduce(n):
    exploded = True
    splited = False
    while(exploded or splited):
        splited = False
        (exploded, explosion, n) = explode(0, n)
        if(not exploded):
            splited, n = split(n)
    return n


def magns(homework):
    magns = []
    for number in homework:
        for other_number in homework:
            if(number != other_number):
                magns.append(magnitude(sum_number(number, other_number)))

    return magns
