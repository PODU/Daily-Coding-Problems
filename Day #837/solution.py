# Day 837: Sentence checker over a character stream.
# Accumulate chars until a terminal mark, then validate the buffered sentence by regex; print if valid.
# O(N) over the stream; O(L) per sentence buffer.
import re

TERMINALS = ".?!‽"  # . ? ! ‽
# start: capital + (lowercase letters)*; words separated by single spaces; optional separators , ; :
PATTERN = re.compile(r"^[A-Z][a-z]*[,;:]?( [a-z]+[,;:]?)*[.?!‽]$")


def check_stream(stream):
    results = []
    buf = []
    for ch in stream:
        buf.append(ch)
        if ch in TERMINALS:
            sentence = "".join(buf).lstrip()  # trim single leading space between sentences
            if PATTERN.match(sentence):
                results.append(sentence)
            buf = []
    return results


if __name__ == "__main__":
    stream = "Hello, world. this is wrong. The cat sat."
    for s in check_stream(stream):
        print(s)
