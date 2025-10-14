# Day 431: Sentence validator via finite-state-machine scan.
# Mirrors regex ^[A-Z][a-z]*([,;:]? [a-z]+)*[,;:]?[.?!‽]$  (FSM, no backtracking needed).
# O(n) time, O(1) extra space per sentence.

TERM = set('.?!‽')   # terminal marks: . ? ! interrobang
SEP = set(',;:')          # separators

def is_valid_sentence(s):
    n = len(s)
    if n == 0:
        return False
    if not ('A' <= s[0] <= 'Z'):          # rule 1: start with capital
        return False
    i = 1
    while i < n and 'a' <= s[i] <= 'z':    # capital then [a-z]*
        i += 1
    while True:                            # ([,;:]? ' ' [a-z]+)* : single-space-separated words
        j = i
        if j < n and s[j] in SEP:
            j += 1
        if j < n and s[j] == ' ':
            j += 1
            if j < n and 'a' <= s[j] <= 'z':
                while j < n and 'a' <= s[j] <= 'z':
                    j += 1
                i = j
                continue
        break
    if i < n and s[i] in SEP:              # optional trailing separator
        i += 1
    return i == n - 1 and s[i] in TERM     # ends with a terminal mark right after a word


def main():
    tests = ["The quick brown fox.", "hello world.", "Hello  world.",
             "Hello world", "Hi there, friend!"]
    for t in tests:
        print(("Valid: " if is_valid_sentence(t) else "Invalid: ") + t)


if __name__ == "__main__":
    main()
