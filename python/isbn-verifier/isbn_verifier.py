def is_valid(isbn: str) -> bool:
    if isbn == '':
        return False

    isbn = isbn.replace('-', '')
    if len(isbn) != 10:
        return False

    if 'X' in isbn and isbn.index('X') != 9:
        return False

    sum = 0
    for (index, value) in enumerate(isbn):
        if not value.isdigit() and value != 'X':
            return False

        cur = ord(value) - ord('0')
        if value == 'X':
            cur = 10

        sum += (10 - index) * cur

    return sum % 11 == 0
