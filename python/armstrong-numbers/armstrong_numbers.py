def is_armstrong_number(number: int) -> bool:
    sum = 0
    number_cp = number
    lens = len(str(number))
    while number_cp > 0 :
        sum += (number_cp % 10) ** lens
        number_cp //= 10

    return sum == number
