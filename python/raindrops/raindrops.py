def convert(number):
    ans = ""
    if number % 3 == 0:
        ans += "Pling"
    if number % 5 == 0:
        ans += "Plang"
    if number % 7 == 0:
        ans += "Plong"

    return ans or str(number)
