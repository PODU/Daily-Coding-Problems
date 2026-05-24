# Day 1551: Sentence validator over a char stream: split on terminal marks, validate each candidate.
# isValid checks capital start, lowercase/separators body, single spaces, terminal end. Time O(n).

TERMINALS = ".?!‽"   # . ? ! ‽
SEPARATORS = ",;:"


def isValid(sentence):
    n = len(sentence)
    if n < 2:
        return False
    # 1. Starts with capital letter, then lowercase letter or space.
    if not ('A' <= sentence[0] <= 'Z'):
        return False
    if not (('a' <= sentence[1] <= 'z') or sentence[1] == ' '):
        return False
    # 4. Ends with a terminal mark.
    if sentence[-1] not in TERMINALS:
        return False
    # Char before the terminal must be part of a word (lowercase or capital), not space/separator.
    if not (('a' <= sentence[-2] <= 'z') or ('A' <= sentence[-2] <= 'Z')):
        return False
    for idx in range(1, n - 1):
        c = sentence[idx]
        if 'a' <= c <= 'z' or c in SEPARATORS:
            continue
        if c == ' ':
            # 3. No double spaces.
            if sentence[idx - 1] == ' ':
                return False
            continue
        # Any other char (capitals beyond first, terminals mid-sentence, etc.) is invalid.
        return False
    return True


def stream_driver(stream):
    results = []
    buf = []
    for ch in stream:
        buf.append(ch)
        if ch in TERMINALS:
            candidate = "".join(buf).strip()
            if isValid(candidate):
                results.append(candidate)
            buf = []
    return results


def main():
    stream = "Hello world. this is bad."
    for sentence in stream_driver(stream):
        print(sentence)


if __name__ == "__main__":
    main()
