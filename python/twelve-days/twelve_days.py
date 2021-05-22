def recite(start_verse, end_verse):
    ans = []
    days = {
        1: "first",
        2: "second",
        3: "third",
        4: "fourth",
        5: "fifth",
        6: "sixth",
        7: "seventh",
        8: "eighth",
        9: "ninth",
        10: "tenth",
        11: "eleventh",
        12: "twelfth",
    }
    objects = [
        "twelve Drummers Drumming, ",
        "eleven Pipers Piping, ",
        "ten Lords-a-Leaping, ",
        "nine Ladies Dancing, ",
        "eight Maids-a-Milking, ",
        "seven Swans-a-Swimming, ",
        "six Geese-a-Laying, ",
        "five Gold Rings, ",
        "four Calling Birds, ",
        "three French Hens, ",
        "two Turtle Doves, ",
        "a Partridge in a Pear Tree.",
    ]

    for day in range(start_verse, end_verse + 1):
        strings = "On the %s day of Christmas my true love gave to me: " % days[day]
        if day == 1:
            ans.append(strings + objects[-1])
        else:
            for i in range(12 - day, len(objects)):
                if i != 11:
                    strings += objects[i]
                else:
                    strings += "and " + objects[i]
            ans.append(strings)

    return ans
