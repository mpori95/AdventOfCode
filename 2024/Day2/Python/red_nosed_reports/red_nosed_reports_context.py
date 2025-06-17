from report import Report

class RedNosedReportsContext:
    def _init__(self, reports: list[Report], max_difference: int):
        self.reports = reports
        self.max_difference = max_difference