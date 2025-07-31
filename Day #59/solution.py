# Day 59: File sync over low-bandwidth link, rsync-style.
# Receiver sends per-block (weak rolling + strong) checksums of its OLD file;
# sender scans NEW file with a rolling checksum, emitting COPY(block) tokens for
# matches and LITERAL bytes otherwise -> only differing bytes cross the wire.
# Time: O(n) expected (rolling hash), bandwidth ~ size of the changes.
M = 1 << 16
B = 4  # block size (small for the demo)


def weak_full(data, start, end):
    a = b = 0
    for i in range(start, end):
        a = (a + data[i]) % M
        b = (b + (end - i) * data[i]) % M
    return a, b


def fnv1a(data, start, end):
    h = 1469598103934665603
    for i in range(start, end):
        h ^= data[i]
        h = (h * 1099511628211) & 0xFFFFFFFFFFFFFFFF
    return h


def block_signatures(old):
    """Receiver side: weak->{strong: index} for each fixed block of OLD file."""
    sigs = {}
    n = len(old) // B
    for i in range(n):
        s, e = i * B, i * B + B
        a, b = weak_full(old, s, e)
        weak = (b << 16) | a
        sigs.setdefault(weak, {})[fnv1a(old, s, e)] = i
    return sigs


def diff(new, sigs):
    """Sender side: produce a delta of LITERAL/COPY tokens for NEW file."""
    tokens, literal = [], []
    pos, n = 0, len(new)
    a = b = 0
    have_window = False
    while pos + B <= n:
        if not have_window:
            a, b = weak_full(new, pos, pos + B)
            have_window = True
        weak = (b << 16) | a
        idx = sigs.get(weak, {}).get(fnv1a(new, pos, pos + B)) if weak in sigs else None
        if idx is not None:
            if literal:
                tokens.append(("L", bytes(literal)))
                literal = []
            tokens.append(("C", idx))
            pos += B
            have_window = False
        else:
            literal.append(new[pos])
            # roll window [pos, pos+B) -> [pos+1, pos+1+B)
            old_first = new[pos]
            a = (a - old_first + new[pos + B]) % M
            b = (b - B * old_first + a) % M
            pos += 1
    literal.extend(new[pos:])
    if literal:
        tokens.append(("L", bytes(literal)))
    return tokens


def apply_delta(old, tokens):
    out = bytearray()
    for kind, payload in tokens:
        if kind == "C":
            out += old[payload * B: payload * B + B]
        else:
            out += payload
    return bytes(out)


if __name__ == "__main__":
    old = b"the quick brown fox jumps over the lazy dog"
    new = b"the quick brown cat jumps over the lazy dog"
    sigs = block_signatures(old)
    tokens = diff(new, sigs)
    rebuilt = apply_delta(old, tokens)
    literal_bytes = sum(len(p) for k, p in tokens if k == "L")
    copied = sum(1 for k, _ in tokens if k == "C")
    print("synced:", rebuilt == new)
    print("literal bytes sent:", literal_bytes, "of", len(new))
    print("blocks reused:", copied)
