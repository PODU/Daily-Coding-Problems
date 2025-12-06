# Day 704: Count valid Android unlock patterns of length N on a 3x3 keypad.
# Approach: DFS with a "skip" table (a jump is legal only if the middle key was
# already used); symmetry cuts the work. Time O(N!) worst but tiny (<=9 keys).

skip = [[0] * 10 for _ in range(10)]
skip[1][3] = skip[3][1] = 2
skip[1][7] = skip[7][1] = 4
skip[3][9] = skip[9][3] = 6
skip[7][9] = skip[9][7] = 8
skip[1][9] = skip[9][1] = skip[3][7] = skip[7][3] = 5
skip[2][8] = skip[8][2] = skip[4][6] = skip[6][4] = 5


def number_of_patterns(n):
    visited = [False] * 10

    def dfs(cur, remaining):
        if remaining == 0:
            return 1
        visited[cur] = True
        count = 0
        for nx in range(1, 10):
            if not visited[nx]:
                mid = skip[cur][nx]
                if mid == 0 or visited[mid]:
                    count += dfs(nx, remaining - 1)
        visited[cur] = False
        return count

    return 4 * dfs(1, n - 1) + 4 * dfs(2, n - 1) + dfs(5, n - 1)


if __name__ == "__main__":
    print(number_of_patterns(4))  # 1624
