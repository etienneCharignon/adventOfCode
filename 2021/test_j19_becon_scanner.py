from inputj19 import simple_scanner_0, simple_scanner_1


def difference(p1, p2):
    difference = []
    for i1, i2 in zip(p1, p2):
        difference.append(i1-i2)
    return difference


def shift(beacons, delta):
    shifted = []
    for position in beacons:
        shifted.append()

    return shifted


def overlapping(s1, s2, matching):
    for b1 in s1:
        for b2 in s2:
            delta = difference(b1, b2)
            shifted_s2 = shift(s2, delta)
            commons = s1 - (s1 - shifted_s2)
            if(len(commons) >= matching):
                return commons

    return []


def xtest_direct_overlapping():
    assert overlapping(simple_scanner_0, simple_scanner_1, 3) == [[0, 2], [3, 3], [4, 1]]
