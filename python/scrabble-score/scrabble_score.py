from collections import Counter

SCORES = {
    0: [""],
    1: ['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T'],
    2: ['D', 'G'],
    3: ['B', 'C', 'M', 'P'],
    4: ['F', 'H', 'V', 'W', 'Y'],
    5: ['K'],
    8: ['J', 'X'],
    10: ['Q', 'Z']
}


def score(word: str) -> int:
    c = Counter(word.upper()).most_common()
    score = 0
    for (char, count) in c:
        for key, value in SCORES.items():
            if char in value:
                score += count * key
                break

    return score
