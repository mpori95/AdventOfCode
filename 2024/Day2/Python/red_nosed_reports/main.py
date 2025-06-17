from report import Report

def main():
    reports = collect_reports()
    fix_problem_dampener(reports)

    print(len([report for report in reports if report.is_safe == True]))

def collect_reports() -> list[Report]:

    reports = []

    with open("unusual_data.txt", "r") as file:
        for report in file:
            levels = list(map(int, report.strip().split()))

            max_difference = 3

            is_increasing = is_level_increasing(levels, max_difference)
            
            is_decreasing = is_level_decreasing(levels, max_difference)
            
            if (is_increasing or is_decreasing):
                reports.append(Report(list(map(int, levels)), True))
            else:
                reports.append(Report(list(map(int, levels)), False))

    return reports

def fix_problem_dampener(reports: list[Report]):
    
    max_difference = 3

    for report in [report for report in reports if report.is_safe == False]:

        for level_index in range(len(report.levels)):
            levels_copy = report.levels.copy()

            del levels_copy[level_index]

            for level in range(len(levels_copy)):

                is_increasing = is_level_increasing(levels_copy, max_difference)

                is_decreasing = is_level_decreasing(levels_copy, max_difference)

                if (is_increasing or is_decreasing):
                    report.is_safe = True
            
def is_level_increasing(levels: list[int], max_difference: int) -> bool:
    return all(level < next_level and next_level - level <= max_difference for level, next_level in zip(levels, levels[1:]))

def is_level_decreasing(levels: list[int], max_difference: int) -> bool:
   return all(level > next_level and level - next_level <= max_difference for level, next_level in zip(levels, levels[1:]))

if __name__ == "__main__":
    main()