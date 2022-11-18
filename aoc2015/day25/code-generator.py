def run(target_row, target_col):
    last_val = 20151125
    row = 2
    col = 1

    while True:
        last_val = (last_val * 252533) % 33554393
        if row == target_row and col == target_col:
            return last_val

        if row % 100 == 0 and col == 1:
            print(f"row={row}, col={col}, value={last_val}")

        row -= 1
        col += 1
        if row == 0:
            row = col
            col = 1

if __name__ == "__main__":
    row = 2978
    column = 3083
    print(run(row, column))
