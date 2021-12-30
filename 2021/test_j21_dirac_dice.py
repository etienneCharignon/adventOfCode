sample_positions = [3, 7]
input = [2, 3]


def roll_player(dice, scores, positions, player):
    positions[player] = (positions[player] + (dice + 1) + (dice + 2) + (dice + 3)) % 10
    scores[player] += positions[player] + 1
    return dice + 3


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


def compute_all_possible_dice_run():
    runs = {}
    for dice1 in range(1, 4):
        for dice2 in range(1, 4):
            for dice3 in range(1, 4):
                dices = dice1 + dice2 + dice3
                if(dices in runs):
                    runs[dices] += 1
                else:
                    runs[dices] = 1
    return runs


def test_compute_all_posible_dice_run():
    assert compute_all_possible_dice_run() == {
        3: 1,
        4: 3,
        5: 6,
        6: 7,
        7: 6,
        8: 3,
        9: 1
    }


def play_dice(player, dice, dice_combination, u, count, new_universes):
    scores, positions = u
    scores = list(scores)
    positions = list(positions)
    positions[player] = (positions[player] + dice) % 10
    scores[player] += positions[player] + 1
    if(scores[player] >= 21):
        scores[player] = 21
        scores[1 - player] = 0
        positions = [0, 0]
    new_univers = (tuple(scores), tuple(positions))
    if(new_univers in new_universes):
        new_universes[new_univers] += count * dice_combination
    else:
        new_universes[new_univers] = count * dice_combination


def test_play_dice():
    new_universes = {}
    play_dice(0, 3, 1, ((0, 0), (1, 2)), 1, new_universes)
    assert new_universes == {((5, 0), (4, 2)): 1}
    new_universes = {}
    play_dice(1, 6, 7, ((5, 0), (4, 2)), 1, new_universes)
    assert new_universes == {((5, 9), (4, 8)): 7}
    new_universes = {}
    play_dice(1, 6, 7, ((5, 0), (4, 2)), 10, new_universes)
    assert new_universes == {((5, 9), (4, 8)): 70}
    new_universes = {((5, 9), (4, 8)): 7}
    play_dice(1, 6, 7, ((5, 0), (4, 2)), 10, new_universes)
    assert new_universes == {((5, 9), (4, 8)): 77}
    new_universes = {}
    play_dice(0, 3, 1, ((18, 20), (9, 2)), 100, new_universes)
    assert new_universes == {((21, 0), (0, 0)): 100}
    new_universes = {}
    play_dice(1, 4, 3, ((20, 18), (2, 9)), 100, new_universes)
    assert new_universes == {((0, 21), (0, 0)): 300}


def run_dice_player(player, dice_runs, universes):
    new_universes = {}
    won_universes = [((21, 0), (0, 0)), ((0, 21), (0, 0))]
    for u in won_universes:
        if(u in universes):
            new_universes[u] = universes[u]
            # print(f'winner player {new_universes}')
    for u, count in universes.items():
        if(u not in won_universes):
            for dice, combinations in dice_runs.items():
                play_dice(player, dice, combinations, u, count, new_universes)
    return new_universes


def test_run_dice_player():
    universes = {((0, 0), (1, 2)): 10}
    dice_runs = compute_all_possible_dice_run()
    universes = run_dice_player(0, dice_runs, universes)
    # print(universes)
    assert sum(universes.values()) == 10*27
    universes = run_dice_player(1, dice_runs, universes)
    assert sum(universes.values()) == 10*27*27


def roll_quantum(positions):
    universes = {((0, 0), positions): 1}
    dice_runs = compute_all_possible_dice_run()
    while(len(universes) != 2):
        universes = run_dice_player(0, dice_runs, universes)
        print(f"different : {len(universes)}, total : {sum(universes.values())}")
        if(len(universes) != 2):
            universes = run_dice_player(1, dice_runs, universes)
            print(f"different : {len(universes)}, total : {sum(universes.values())}")

    print(universes)
    return universes.values()


def test_roll_quantum():
    assert roll_quantum(tuple(sample_positions)) == [444356092776315, 341960390180808]
# 93726416205179
# 49950658789496
# 444356092776315
# 341960390180808


# print(roll_quantum(tuple(sample_positions)))
