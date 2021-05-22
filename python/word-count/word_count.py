import re


def count_words(sentence: str):
    sentence = re.findall(r'\w+\'?\w+|\w', sentence.lower().replace('_', ' '))
    dict = {}
    for word in sentence:
        if dict.get(word):
            dict[word] += 1
            continue
        dict.setdefault(word, 1)

    # print(dict)
    return dict


if __name__ == "__main__":
    count_words("Joe can't tell between app, apple and a.")
