def encode(string: str):
    ans: str = ""
    count: int = 0
    prev: str = ""
    for c in string:
        if c != prev:
            if count > 1:
                ans += str(count) + prev
            else:
                ans += prev
            count = 1
            prev = c
        else:
            count += 1

    if count > 1:
        ans += str(count) + prev
    else:
        ans += prev

    return ans



def decode(string: str):
    ans: str = ""
    count: str = ""
    for c in string:
        if c.isdigit():
            count += c
        else:
            if count != "":
                ans += c * int(count)
                count = ""
            else:
                ans += c
    return ans
