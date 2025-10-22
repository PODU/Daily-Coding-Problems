# Day 478: rsync-style sync: split the destination (old) file into fixed blocks, index each by a weak
# rolling Adler-32 checksum + a strong FNV hash. Slide a rolling window over the source (new) file
# to find matches, emitting COPY(block index) / LITERAL(diff bytes) tokens; only literals travel
# the wire. Reconstruct new = old blocks + literals. Time O(n) (constant block size), only diffs sent.
M = 65521
BLOCK = 4


def fnv(data, start, length):
    h = 0xcbf29ce484222325
    for k in range(start, start + length):
        h ^= data[k]
        h = (h * 0x100000001b3) & 0xFFFFFFFFFFFFFFFF
    return h


def weak_full(data, i, L):
    a = b = 0
    for k in range(i, i + L):
        a = (a + data[k]) % M
        b = (b + a) % M
    return a, b


def build_index(old):
    idx = {}
    nb = len(old) // BLOCK
    for bi in range(nb):
        s = bi * BLOCK
        a, b = weak_full(old, s, BLOCK)
        weak = a + (b << 16)
        idx.setdefault(weak, []).append((bi, fnv(old, s, BLOCK)))
    return idx


def sync(old, new):
    idx = build_index(old)
    tokens = []
    lit = bytearray()
    n = len(new)
    pos = 0
    a = b = 0
    have = False
    while pos < n:
        if pos + BLOCK <= n:
            if not have:
                a, b = weak_full(new, pos, BLOCK)
                have = True
            weak = a + (b << 16)
            matched = -1
            if weak in idx:
                strong = fnv(new, pos, BLOCK)
                for bi, st in idx[weak]:
                    if st == strong:
                        matched = bi
                        break
            if matched >= 0:
                if lit:
                    tokens.append(("L", bytes(lit)))
                    lit = bytearray()
                tokens.append(("C", matched))
                pos += BLOCK
                have = False
                continue
            lit.append(new[pos])
            if pos + BLOCK < n:                       # roll window forward by one byte
                out_b, in_b = new[pos], new[pos + BLOCK]
                a = (a - out_b + in_b) % M
                b = (b - BLOCK * out_b + a) % M
            else:
                have = False
            pos += 1
        else:
            lit.append(new[pos])
            pos += 1
    if lit:
        tokens.append(("L", bytes(lit)))
    return tokens


def reconstruct(old, tokens):
    out = bytearray()
    for t in tokens:
        if t[0] == "C":
            s = t[1] * BLOCK
            out += old[s:s + BLOCK]
        else:
            out += t[1]
    return bytes(out)


if __name__ == "__main__":
    old = b"the quick brown fox jumps over the lazy dog"
    new = b"the quick BROWN fox jumps over the lazy cat"
    tokens = sync(old, new)
    matched = sum(1 for t in tokens if t[0] == "C")
    literal_bytes = sum(len(t[1]) for t in tokens if t[0] == "L")
    recon = reconstruct(old, tokens)
    print("Matched blocks: %d (%d bytes), Literal bytes: %d" % (matched, matched * BLOCK, literal_bytes))
    print("Reconstructed: " + recon.decode())
    print("Equals new file: " + str(recon == new))
