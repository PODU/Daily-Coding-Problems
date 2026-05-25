# Day 1562: Sort mice and holes, pair by index, answer = max |mice[i]-holes[i]|. Time O(n log n), Space O(1).


def assign(mice, holes):
    mice = sorted(mice)
    holes = sorted(holes)
    return max(abs(m - h) for m, h in zip(mice, holes))


def main():
    mice = [1, 4, 9, 15]
    holes = [10, -5, 0, 16]
    print(assign(mice, holes))


if __name__ == "__main__":
    main()
