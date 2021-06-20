from typing import List


class Tournament:
    def __init__(self, name: str):
        self.name = name
        self.mp = 0
        self.w = 0
        self.d = 0
        self.l = 0
        self.p = 0

    def is_win(self):
        self.mp += 1
        self.w += 1
        self.p += 3

    def is_draw(self):
        self.mp += 1
        self.d += 1
        self.p += 1

    def is_loss(self):
        self.mp += 1
        self.l += 1

    def pos_state(self, state):
        if state == "win":
            self.is_win()
        elif state == "draw":
            self.is_draw()
        elif state == "loss":
            self.is_loss()

    def neg_state(self, state):
        if state == "win":
            self.is_loss()
        elif state == "draw":
            self.is_draw()
        elif state == "loss":
            self.is_win()


def tally(rows: List[str]) -> List[str]:
    ans = []
    ans.append("Team                           | MP |  W |  D |  L |  P")
    players: dict[str, Tournament] = dict()
    for row in rows:
        row = row.split(";")
        first, second, state = row[0], row[1], row[2]
        if first not in players.keys():
            players[first] = Tournament(first)
        if second not in players.keys():
            players[second] = Tournament(second)

        players.get(first).pos_state(state)
        players.get(second).neg_state(state)
    players = sorted(
        players.items(), key=lambda x: (-x[1].p, x[0]))
    for _, v in players:
        s = "{:<31}| {:>2} | {:>2} | {:>2} | {:>2} | {:>2}".format(
            v.name, v.mp, v.w, v.d, v.l, v.p)
        ans.append(s)

    return ans
