from collections import defaultdict
from dataclasses import dataclass
from typing import DefaultDict


@dataclass
class Problem:
    l1: list
    l2: list

    @property
    def total_dist(self) -> int:
        total = 0
        for x1, x2 in zip(sorted(self.l1), sorted(self.l2)):
            total += abs(x1 - x2)
        return total

    @property
    def sim_score(self) -> int:
        l2_freq_map = Problem.get_freq_map(self.l2)
        score = 0
        for x in self.l1:
            score += x * l2_freq_map[x]
        return score

    @staticmethod
    def get_freq_map(nums: list) -> DefaultDict[int, int]:
        freq_map = defaultdict(int)
        for x in nums:
            freq_map[x] += 1
        return freq_map


def parse_input(filename: str) -> Problem:
    l1, l2 = [], []
    with open(filename, "r") as f:
        while line := f.readline():
            x1, x2 = map(int, line.split())
            l1.append(x1)
            l2.append(x2)

    return Problem(l1=l1, l2=l2)


if __name__ == "__main__":
    import argparse

    p = argparse.ArgumentParser()
    p.add_argument("--filename", type=str, required=True)

    args = p.parse_args()

    filename = args.filename

    prob = parse_input(filename)
    print(f"Total distance: {prob.total_dist}")
    print(f"Similarity score: {prob.sim_score}")
