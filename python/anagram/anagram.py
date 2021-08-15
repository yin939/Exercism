from typing import List


def find_anagrams(word: str, candidates: List[str]) -> List[str]:
    ans = []
    word_lower = word.lower()
    word_sort = list(word_lower)
    word_sort.sort()
    for candidate in candidates:
        candidate_lower = candidate.lower()
        candidate_sort = list(candidate_lower)
        candidate_sort.sort()
        if len(word_lower) == len(candidate) and word_lower != candidate_lower and word_sort == candidate_sort:
            ans.append(candidate)

    return ans
