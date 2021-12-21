sample_positions = [3, 7]
input = [2, 3]


def roll_player(dice, scores, positions, player):
    dice += 1
    positions[player] += dice
    positions[player] %= 10
    print(positions)
    dice += 1
    positions[player] += dice
    positions[player] %= 10
    print(positions)
    dice += 1
    positions[player] += dice
    positions[player] %= 10
    print(positions)
    scores[player] += positions[player] + 1
    return dice


def roll_game(positions):
    scores = [0, 0]
    rolled = 0
    dice = 0
    while(max(scores) < 1000):
        dice = roll_player(dice, scores, positions, 0)
        rolled += 3

        if(scores[0] >= 1000):
            break

        dice = roll_player(dice, scores, positions, 1)
        rolled += 3

    return min(scores) * rolled


def test_roll_player():
    scores = [0, 0]
    positions = [3, 7]
    assert roll_player(0, scores, positions, 0) == 3
    assert positions[0] == 9
    assert scores[0] == 10


def test_roll_game():
    assert roll_game(sample_positions) == 739785
    assert roll_game(input) == 995904
