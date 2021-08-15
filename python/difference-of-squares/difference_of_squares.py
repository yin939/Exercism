def square_of_sum(number: int) -> int:
    _sum = sum(range(1, number + 1))

    return _sum ** 2


def sum_of_squares(number: int) -> int:
    return sum([i ** 2 for i in range(1, number + 1)])


def difference_of_squares(number: int) -> int:
    return square_of_sum(number) - sum_of_squares(number)
