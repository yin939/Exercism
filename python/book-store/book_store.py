DISCOUNT = {
    1: 1,
    2: 0.95,
    3: 0.9,
    4: 0.8,
    5: 0.75
}


def total(basket):
    sum = 0
    set_count = [0] * 6
    while basket != []:
        s = set(basket)
        l = len(s)
        set_count[l] += 1
        for i in s:
            basket.remove(i)
    while set_count[3] > 0 and set_count[5] > 0:
        set_count[3] -= 1
        set_count[5] -= 1
        set_count[4] += 2

    for i in range(1, 6):
        sum += set_count[i] * i * 800 * DISCOUNT[i]

    return round(sum)
