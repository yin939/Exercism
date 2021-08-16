from typing import List


def slices(series: str, length: int) -> List[str]:
    if length <= 0 or len(series) < length or series == "":
        raise ValueError("Illeagle input...")

    ans: List[str] = []
    for i in range(len(series)):
        if i + length > len(series):
            break
        ans.append(series[i:i+length])

    return ans
