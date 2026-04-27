# Day 1433: Sentence checker over a character stream.
# Approach: finite-state machine validating one sentence at a time. Time: O(n), Space: O(n) buffer.

TERMINALS = set(".?!‽")  # . ? ! ‽
SEPARATORS = set(",;:")


def is_valid_sentence(s):
    n = len(s)
    if n < 2:
        return False
    # Rule 1: capital then lowercase or space.
    if not s[0].isupper():
        return False
    if not (s[1].islower() or s[1] == " "):
        return False

    prev_was_letter = s[0].isalpha()
    for i in range(1, n):
        c = s[i]
        if c in TERMINALS:
            # Rule 4: terminal immediately after a word and ends the sentence.
            if not prev_was_letter:
                return False
            return i == n - 1
        if c == " ":
            if s[i - 1] == " ":  # Rule 3: single space between words
                return False
            prev_was_letter = False
        elif c.islower():
            prev_was_letter = True
        elif c in SEPARATORS:
            prev_was_letter = False
        else:
            return False
    return False  # no terminal mark


def check_stream(chars):
    """Read a stream of characters, splitting on terminal marks, print valid sentences."""
    buffer = []
    for ch in chars:
        buffer.append(ch)
        if ch in TERMINALS:
            sentence = "".join(buffer).strip()
            if is_valid_sentence(sentence):
                print(sentence)
            buffer = []


if __name__ == "__main__":
    tests = [
        "The quick brown fox.",
        "Hello world!",
        "lowercase start.",
        "No terminal mark",
        "Two  spaces here.",
    ]
    for t in tests:
        if is_valid_sentence(t):
            print(t)
