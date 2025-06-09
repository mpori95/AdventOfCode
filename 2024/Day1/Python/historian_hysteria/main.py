from column_data import ColumnData

def main():
    column_data = collect_location_ids()
    # print_location_id_differences(column_data)
    print_left_list_similarities(column_data)

def collect_location_ids():
    """
    Collects location ids from the text file.

    Returns:
        ColumnData: The class containing the location id data.
    """

    first_column = []
    second_column = []

    with open("location_ids.txt", "r") as file:
        for line in file:
            parts = line.strip().split()
            first_column.append(parts[0])
            second_column.append(parts[1])
    
    first_column.sort()
    second_column.sort()

    return ColumnData(first_column, second_column)

def print_location_id_differences(column_data: ColumnData):
    """
    Prints the location id defferences between the two columns.

    Args:
        column_data (ColumnData): The column data containing the location ids.
    """

    column_length = len(column_data.first_column)

    total_difference = 0

    for i in range(column_length):
        left_value = int(column_data.first_column[i])
        right_value = int(column_data.second_column[i])

        difference = left_value - right_value if left_value > right_value else right_value - left_value

        total_difference += difference

    print(total_difference)


def print_left_list_similarities(column_data: ColumnData):
    """
    Prints the similarity score of the left list.

    Args:
        column_data (ColumnData): The column data containing the location ids.
    """

    total_similarity_score = 0

    for item in column_data.first_column:
        number_of_occurances = column_data.second_column.count(item)
        total_similarity_score += int(item) * number_of_occurances

    print(total_similarity_score)

if __name__ == "__main__":
    main()