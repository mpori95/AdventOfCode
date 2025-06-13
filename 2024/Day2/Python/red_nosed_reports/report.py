from typing import List

class Report:
    def __init__(self, levels: List[int], is_safe: bool):
        self.levels = levels
        self.is_safe = is_safe