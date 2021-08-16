# Globals for the directions
# Change the values as you see fit
NORTH = 0
EAST = 1
SOUTH = 2
WEST = 3


class Robot:
    def __init__(self, direction=NORTH, x=0, y=0):
        self._direction = direction
        self._coordinates = (x, y)

    @property
    def direction(self):
        return self._direction

    @direction.setter
    def direction(self, direction):
        self._direction = direction

    @property
    def coordinates(self):
        return self._coordinates

    @coordinates.setter
    def coordinates(self, coordinates):
        self._coordinates = coordinates

    def move(self, actions: str):
        for action in actions:
            if action == 'R':
                self.direction = (self.direction + 1) % 4
            elif action == 'L':
                self.direction -= 1
                if self.direction < 0:
                    self.direction = 3
            elif action == 'A':
                (x, y) = self.coordinates
                if self.direction == NORTH:
                    self.coordinates = (x, y+1)
                elif self.direction == EAST:
                    self.coordinates = (x+1, y)
                elif self.direction == SOUTH:
                    self.coordinates = (x, y-1)
                elif self.direction == WEST:
                    self.coordinates = (x-1, y)
