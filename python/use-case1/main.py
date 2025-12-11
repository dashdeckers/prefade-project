from prefade import compute_average

if __name__ == "__main__":
    data = [1.0, 2.0, 3.0, 4.0, 5.0]
    result = compute_average(data)
    print(f"Average of {data}: {result}")

    empty_result = compute_average([])
    print(f"Average of []: {empty_result}")

