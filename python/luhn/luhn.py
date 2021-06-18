class Luhn:
    def __init__(self, card_num: str):
        self.sum = 0
        self.card_num = ''.join(card_num.replace(' ', ''))[::-1]
        self.calculator()

    def valid(self) -> bool:
        if len(self.card_num) > 1 and self.card_num.count('0') == len(self.card_num):
            return True
        return self.sum != 0 and self.sum % 10 == 0

    def calculator(self):
        for (i, v) in enumerate(self.card_num):
            if not v.isdigit():
                self.sum = 1
                break
            if (i + 1) % 2 == 0:
                temp = int(v) * 2
                while temp > 0:
                    self.sum += temp % 10
                    temp //= 10
            else:
                self.sum += int(v)
