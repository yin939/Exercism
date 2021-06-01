def distance(strand_a, strand_b):
    if len(strand_a) != len(strand_b):
        raise ValueError(
            "Meaningful message indicating the source of the error")

    distance = 0
    for i in range(len(strand_a)):
        if strand_a[i] != strand_b[i]:
            distance += 1

    return distance
