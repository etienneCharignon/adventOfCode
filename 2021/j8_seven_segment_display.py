def extract_signals(line):
    return line.split(' | ')[0]


def extract_display(line):
    return line.split(' | ')[1]


def filter_segment(signal, segment):
    return [s for s in segment if s in signal]


def exclude_segment(signal, segment):
    return [s for s in segment if s not in signal]


def filter_signals(signals, display):
    signals = signals.split()
    easy_signals = [digit for digit in signals if len(digit) in [2, 4, 3, 7]]
    hard_signals = [digit for digit in signals if len(digit) not in [2, 4, 3, 7]]
    easy_signals = sorted(easy_signals, key=lambda signal: len(signal))
    hard_signals = sorted(hard_signals, key=lambda signal: -1 * len(signal))
    print(hard_signals)
    digits = {
    }
    for signal in easy_signals:
        print(signal)
        (display, digit) = filter_one_signal(signal, display)
        digits[''.join(sorted(signal))] = digit
        print(display)
    print(digits)
    for signal in hard_signals:
        print(signal)
        (display, digit) = filter_one_signal(signal, display)
        digits[''.join(sorted(signal))] = digit
        print(display)
    print(digits)

    return (display, digits)


def decode_one_row_signals(signals):
    display = {
        'a': ['a', 'b', 'c', 'd', 'e', 'f', 'g'],
        'b': ['a', 'b', 'c', 'd', 'e', 'f', 'g'],
        'c': ['a', 'b', 'c', 'd', 'e', 'f', 'g'],
        'd': ['a', 'b', 'c', 'd', 'e', 'f', 'g'],
        'e': ['a', 'b', 'c', 'd', 'e', 'f', 'g'],
        'f': ['a', 'b', 'c', 'd', 'e', 'f', 'g'],
        'g': ['a', 'b', 'c', 'd', 'e', 'f', 'g']
    }
    return filter_signals(signals, display)


def filter_one_signal(signal, display):
    digit = 0
    if(len(signal) == 2):
        digit = 1
        display['a'] = exclude_segment(signal, display['a'])
        display['b'] = exclude_segment(signal, display['b'])
        display['c'] = filter_segment(signal, display['c'])
        display['d'] = exclude_segment(signal, display['d'])
        display['e'] = exclude_segment(signal, display['e'])
        display['f'] = filter_segment(signal, display['f'])
        display['g'] = exclude_segment(signal, display['g'])
    elif(len(signal) == 4):
        digit = 4
        display['a'] = exclude_segment(signal, display['a'])
        display['b'] = filter_segment(signal, display['b'])
        display['c'] = filter_segment(signal, display['c'])
        display['d'] = filter_segment(signal, display['d'])
        display['e'] = exclude_segment(signal, display['e'])
        display['f'] = filter_segment(signal, display['f'])
        display['g'] = exclude_segment(signal, display['g'])
    elif(len(signal) == 3):
        digit = 7
        display['a'] = filter_segment(signal, display['a'])
        display['b'] = exclude_segment(signal, display['b'])
        display['c'] = filter_segment(signal, display['c'])
        display['d'] = exclude_segment(signal, display['d'])
        display['e'] = exclude_segment(signal, display['e'])
        display['f'] = filter_segment(signal, display['f'])
        display['g'] = exclude_segment(signal, display['g'])
    elif(len(signal) == 7):
        digit = 8
        display['a'] = filter_segment(signal, display['a'])
        display['b'] = filter_segment(signal, display['b'])
        display['c'] = filter_segment(signal, display['c'])
        display['d'] = filter_segment(signal, display['d'])
        display['e'] = filter_segment(signal, display['e'])
        display['f'] = filter_segment(signal, display['f'])
        display['g'] = filter_segment(signal, display['g'])
    elif(len(signal) == 6):
        in_segment_d = [s for s in display['d'] if s in signal]
        if(len(in_segment_d) == 2):
            in_segment_c = [s for s in display['c'] if s in signal]
            if(len(in_segment_c) == 2):
                digit = 9
            else:
                digit = 6
        else:
            digit = 0
    elif(len(signal) == 5):
        # c'est un 2 un 3 ou un 5

        contient_un = len([s for s in signal if s not in display['c']]) == 3
        in_segment_b = [s for s in display['b'] if s in signal]
        in_segment_e = [s for s in display['e'] if s in signal]
        if(contient_un):
            digit = 3
        if(len(in_segment_b) == 2):
            print("c'est un 5")
            digit = 5
        elif(len(in_segment_e) == 2):
            print("c'est un 2")
            digit = 2

    return (display, digit)
