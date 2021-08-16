from typing import List


def primes(limit: int) -> List[int]:
    if limit < 2:
        return []

    sieve = [True] * (limit + 1)
    m = 2

    while m * m < limit:
        if sieve[m] == True:
            for i in range(m * 2, limit+1, m):
                sieve[i] = False
        m += 1

    return [i for i in range(2, limit+1) if sieve[i] == True]
