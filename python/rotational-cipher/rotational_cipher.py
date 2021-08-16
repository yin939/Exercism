from string import ascii_letters as al, ascii_uppercase as au


def rotate(text: str, key: int) -> str:
    text = list(text)
    for (i, c) in enumerate(text):
        if c.isalpha():
            if c == c.lower():
                text[i] = al[(al.index(c) + key )% 26]
            elif c == c.upper():
                text[i] = au[(au.index(c) + key) % 26]

    return "".join(text)