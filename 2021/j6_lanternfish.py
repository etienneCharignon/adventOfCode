def next_generation(world):
    new_fish = []

    def decrease_day(fish):
        if(fish == 0):
            new_fish.append(8)
            return 6
        return fish - 1

    result = list(map(decrease_day, world))
    result += new_fish
    return result
