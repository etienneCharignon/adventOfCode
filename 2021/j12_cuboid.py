def length(axe):
    return axe[1] - axe[0] + 1


def volume(cuboid):
    x, y, z = cuboid
    return length(x) * length(y) * length(z)


def intersection(cuboids1, cuboids2):
    x1, y1, z1 = cuboids1
    x2, y2, z2 = cuboids2
    return (
        (max(x1[0], x2[0]), min(x1[1], x2[1])),
        (max(y1[0], y2[0]), min(y1[1], y2[1])),
        (max(z1[0], z2[0]), min(z1[1], z2[1])),
    )


def is_empty(cuboid):
    thicknesses = map(lambda side: side[1] - side[0], cuboid)
    return any(thickness < 0 for thickness in thicknesses)


class LightedCuboid:
    removed = []

    def __init__(self, cuboid):
        self.x = cuboid[0]
        self.y = cuboid[1]
        self.z = cuboid[2]

    def lighted(self):
        return volume((self.x, self.y, self.z))

    def remove(self, cuboid):
        intersection_cuboid = intersection((self.x, self.y, self.z), cuboid)
        if(not is_empty(intersection_cuboid)):
            self.removed.append(intersection_cuboid)
