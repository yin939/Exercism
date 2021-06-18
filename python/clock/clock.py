DAY_MINS = 60 * 24


class Clock:
    def __init__(self, hour, minute):
        self.minute = (minute + hour * 60) % DAY_MINS

    def __repr__(self):
        (hour, minute) = self.handle()
        return str(hour).zfill(2) + ':' + str(minute).zfill(2)

    def __eq__(self, other):
        return self.minute == other.minute

    def __add__(self, minutes):
        self.minute += minutes
        self.minute %= DAY_MINS
        return self.__repr__()

    def __sub__(self, minutes):
        self.minute -= minutes
        self.minute %= DAY_MINS
        return self.__repr__()

    def handle(self):
        if self.minute < 0:
            return ((DAY_MINS + self.minute) // 60, (DAY_MINS+self.minute) % 60)
        return (self.minute // 60, self.minute % 60)
