from datetime import date
import calendar


class MeetupDayException(Exception):
    pass


def meetup(year: int, month: int, week: str, day_of_week: str) -> date:
    date_index = list(calendar.day_name).index(day_of_week)

    # If the week is "teenth", iterate through dates 13 to 19
    if week == "teenth":
        for i in range(13, 20):
            if calendar.weekday(year, month, i) == date_index:
                return date(year, month, i)

    if week == "last":
        order = 0
    else:
        order = int(week[0])

    # Iterate through dates in month to find dates that match the day_of_week
    selected_days = [date for date in range(1, calendar.monthrange(
        year, month)[1] + 1) if calendar.weekday(year, month, date) == date_index]
    if order - 1 >= len(selected_days):
        raise MeetupDayException("Date doesn't exit.")

    return date(year, month, selected_days[order - 1])
