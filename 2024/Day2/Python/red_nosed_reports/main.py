from report import Report

def main():
    collect_reports()

def collect_reports():

    safe_reports = 0

    reports = []

    with open("unusual_data.txt", "r") as file:
        for report in file:
            levels = report.strip().split()

            max_difference = 3

            is_increasing = all(
                is_pair_increasing(int(level), int(next_level), max_difference)
                  for level, next_level in zip(levels, levels[1:]))
            
            is_decreasing = all(
                is_pair_decreasing(int(level), int(next_level), max_difference)
                  for level, next_level in zip(levels, levels[1:]))
            
            if (is_increasing or is_decreasing):
                reports.append(Report(int(levels), True))
                

        
        print(safe_reports)
            
def is_pair_increasing(level: int, next_level: int, max_difference: int):
    return level < next_level and next_level - level <= max_difference

def is_pair_decreasing(level: int, next_level: int, max_difference: int):
    return level > next_level and level - next_level <= max_difference
   
if __name__ == "__main__":
    main()