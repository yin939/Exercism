
def is_isogram(string: str):

    d = dict()
    string = string.lower()
    for str in string:
        if str != '-' and str != ' ':
            if str in d.keys():
                return False
            d[str] = True
    return True
