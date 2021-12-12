from samplej8 import input
from inputj8 import rinput
from j8_seven_segment_display import filter_one_signal, decode_one_row_signals


def extract_signals(line):
    return line.split(' | ')[0]


def extract_display(line):
    return line.split(' | ')[1]


def counting_easy_digits(display):
    digits = display.split()
    return len([digit for digit in digits if len(digit) in [2, 4, 3, 7]])


def counting_all_easy_digits(input):
    return sum(map(counting_easy_digits, map(extract_display, input.split('\n'))))


def test_counting_easy_digits():
    assert counting_easy_digits('fdgacbe') == 1
    assert counting_easy_digits('fdgacbe cefdb cefbgd gcbe') == 2
    assert counting_easy_digits('fcgedb cgb dgebacf gc') == 3


def test_counting_all_easy_digits():
    assert counting_all_easy_digits(input) == 26
    assert counting_all_easy_digits(rinput) == 247


def test_filter_one_signal():
    display = {
        'a': ['a', 'b', 'c', 'd', 'e', 'f', 'g'],
        'b': ['a', 'b', 'c', 'd', 'e', 'f', 'g'],
        'c': ['a', 'b', 'c', 'd', 'e', 'f', 'g'],
        'd': ['a', 'b', 'c', 'd', 'e', 'f', 'g'],
        'e': ['a', 'b', 'c', 'd', 'e', 'f', 'g'],
        'f': ['a', 'b', 'c', 'd', 'e', 'f', 'g'],
        'g': ['a', 'b', 'c', 'd', 'e', 'f', 'g']
    }
    assert filter_one_signal('dab', display) == ({
        'a': ['a', 'b', 'd'],
        'b': ['c', 'e', 'f', 'g'],
        'c': ['a', 'b', 'd'],
        'd': ['c', 'e', 'f', 'g'],
        'e': ['c', 'e', 'f', 'g'],
        'f': ['a', 'b', 'd'],
        'g': ['c', 'e', 'f', 'g']
    }, 7)


def test_filter_one_signal_4():
    display = {
        'a': ['a', 'b', 'c', 'd', 'e', 'f', 'g'],
        'b': ['a', 'b', 'c', 'd', 'e', 'f', 'g'],
        'c': ['a', 'b', 'c', 'd', 'e', 'f', 'g'],
        'd': ['a', 'b', 'c', 'd', 'e', 'f', 'g'],
        'e': ['a', 'b', 'c', 'd', 'e', 'f', 'g'],
        'f': ['a', 'b', 'c', 'd', 'e', 'f', 'g'],
        'g': ['a', 'b', 'c', 'd', 'e', 'f', 'g']
    }
    assert filter_one_signal('eafb', display) == ({
        'a': ['c', 'd', 'g'],
        'b': ['a', 'b', 'e', 'f'],
        'c': ['a', 'b', 'e', 'f'],
        'd': ['a', 'b', 'e', 'f'],
        'e': ['c', 'd', 'g'],
        'f': ['a', 'b', 'e', 'f'],
        'g': ['c', 'd', 'g']
    }, 4)


def test_decode_one_row_signal():
    display = decode_one_row_signals('acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab')
    assert display[1] == {
        'ab': 1,
        'abd': 7,
        'abef': 4,
        'abcdefg': 8,
        'abcdef': 9,
        'abcdeg': 0,
        'bcdefg': 6,
        'abcdf': 3,
        'acdfg': 2,
        'bcdef': 5
    }


def test_decode_other_row_signal():
    display = decode_one_row_signals('edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec')
    assert display[1] == {
        'cg': 1,
        'bcg': 7,
        'cefg': 4,
        'abcdefg': 8,
        'abdefg': 6,
        'abcdfg': 0,
        'bcdefg': 9,
        'bcdeg': 3,
        'bdefg': 5,
        'abcde': 2
    }


def test_parse_signal():
    rows = input.split('\n')
    total = 0
    for row in rows:
        rowSignals = extract_signals(row)
        (display, digits) = decode_one_row_signals(rowSignals)
        four_display = extract_display(row)

        def decode_display(display):
            display = ''.join(sorted(display))
            digit = [digit for signal, digit in digits.items() if signal == display][0]
            return digit

        digits = list(map(str, map(decode_display, four_display.split())))
        display = int(''.join(digits))
        print(display)
        total += display

    assert total == 61229


def test_parse_rsignal():
    rows = rinput.split('\n')
    total = 0
    for row in rows:
        rowSignals = extract_signals(row)
        (display, digits) = decode_one_row_signals(rowSignals)
        four_display = extract_display(row)

        def decode_display(display):
            display = ''.join(sorted(display))
            digit = [digit for signal, digit in digits.items() if signal == display][0]
            return digit

        digits = list(map(str, map(decode_display, four_display.split())))
        display = int(''.join(digits))
        print(display)
        total += display

    assert total == 933305
