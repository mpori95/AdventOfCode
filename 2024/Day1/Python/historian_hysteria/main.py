def main():
    collect_location_ids()

def collect_location_ids():

    first_column = []
    second_column = []

    with open("location_ids.txt", "r") as file:
        for line in file:
            parts = line.strip().split()
            first_column.append(parts[0])
            second_column.append(parts[1])
    
    first_column.sort()
    second_column.sort()

    column_length = len(first_column)

    total_difference = 0

    for i in range(column_length):
        left_value = int(first_column[i])
        right_value = int(second_column[i])

        difference = left_value - right_value if left_value > right_value else right_value - left_value

        total_difference += difference

    print(total_difference)

if __name__ == "__main__":
    main()