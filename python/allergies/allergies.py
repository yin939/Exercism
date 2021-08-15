class Allergies:

    def __init__(self, score):
        self.score = score % 256

    def allergic_to(self, item):
        return item in self.lst

    @property
    def lst(self):
        dic = {
            1: "eggs",
            2: "peanuts",
            4: "shellfish",
            8: "strawberries",
            16: "tomatoes",
            32: "chocolate",
            64: "pollen",
            128: "cats"
        }
        return [v for k, v in dic.items() if self.score & k > 0]
