import re


def abbreviate(words):
    result = re.findall(r'[a-zA-Z\']+', words)

    return "".join([char[0].upper() for char in result])
