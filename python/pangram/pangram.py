import re


def is_pangram(sentence: str):
    if str == '':
        return False

    sentence = re.sub(r'[^a-z]', '', sentence.lower())
    letters: set[str] = set(sentence)
    if len(letters) == 26:
        return True
    return False
