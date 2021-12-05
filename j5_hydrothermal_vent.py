def direction(a, b):
    if(a == b):
        return 1
    return int((b - a) / abs(b - a))
