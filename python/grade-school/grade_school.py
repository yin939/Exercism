
class School:
    def __init__(self):
        self.grades: dict[int, list[str]] = dict()

    def add_student(self, name: str, grade: int):
        if not self.grades.get(grade):
            self.grades.setdefault(grade, [name])
        else:
            self.grades[grade].append(name)

    def roster(self) -> list:
        rosters = []
        for _, v in sorted(self.grades.items(), key=lambda x: x[0]):
            rosters.append(sorted(v))
        return [x for j in rosters for x in j]

    def grade(self, grade_number: int) -> list:
        if not self.grades.get(grade_number):
            return []
        return sorted(self.grades.get(grade_number))
