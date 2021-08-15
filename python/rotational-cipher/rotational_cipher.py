def rotate(text: str, key: int) -> str:
    key = key % 26
    text = list(text)
    for (i, c) in enumerate(text):
        if c >= 'a' and c <= 'z':
            asc = ord(c) + key
            if asc > ord('z'):
                text[i] = chr(asc - ord('z') + 96)
            else:
                text[i] = chr(ord(c) + key)

        if c >= 'A' and c <= 'Z':
            asc = ord(c) + key
            if asc > ord('Z'):
                text[i] = chr(asc - ord('Z') + 64)
            else:
                text[i] = chr(ord(c) + key)

    return "".join(text)