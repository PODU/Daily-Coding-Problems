# Day 784: Word search L-to-R / U-to-D only: scan each row and column for target substring.
# Time O(R*C*L), Space O(max(R,C)).

def find_word(matrix, target):
    R = len(matrix)
    C = len(matrix[0]) if R else 0
    for r in range(R):
        if target in "".join(matrix[r]):
            return True
    for c in range(C):
        col = "".join(matrix[r][c] for r in range(R))
        if target in col:
            return True
    return False


if __name__ == "__main__":
    matrix = [
        ['F', 'A', 'C', 'I'],
        ['O', 'B', 'Q', 'P'],
        ['A', 'N', 'O', 'B'],
        ['M', 'A', 'S', 'S'],
    ]
    print(str(find_word(matrix, "FOAM")).lower())
