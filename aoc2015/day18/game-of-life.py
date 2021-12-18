import sys


def _safe_get_value(arr, i, j):
    if i < 0 or j < 0 or i >= len(arr) or j >= len(arr[0]):
        return 0
    return int(arr[i][j])


def _get_neighbor_count(state, i, j):
    count = 0
    for i_diff in range(-1, 2):
        for j_diff in range(-1, 2):
            if i_diff == j_diff == 0:
                continue
            count += _safe_get_value(state, i + i_diff, j + j_diff)
    return count


def _iterate(state):
    result = [[False for _ in row] for row in state]
    for i, row in enumerate(state):
        for j, val in enumerate(row):
            neighbors = _get_neighbor_count(state, i, j)
            if (val and neighbors in [2,3]) or (not val and neighbors == 3):
                result[i][j] = True
    return result


def _update_corners(state, pin_corners):
    if not pin_corners:
        return state
    state[0][0] = True
    state[0][-1] = True
    state[-1][0] = True
    state[-1][-1] = True
    return state


def game_of_life(state, iterations, pin_corners):
    state = _update_corners(state, pin_corners)
    for _ in range(iterations):
        state = _iterate(state)
        state = _update_corners(state, pin_corners)
    result = 0
    for row in state:
        for val in row:
            result += int(val)
    return result


if __name__ == "__main__":
    state = []
    with open(sys.argv[1], 'r') as f:
        for line in f.readlines():
            row = [int(c == "#") for c in line.strip()]
            state.append(row)

    print(game_of_life(state, 100, bool(sys.argv[2])))
