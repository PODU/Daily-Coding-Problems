# Day 1674: Low-bandwidth file sync (rsync algorithm).
# Receiver sends per-block weak(rolling)+strong checksums; sender emits COPY/LITERAL
# tokens using a rolling hash to find matches. Time O(n) amortized, transfers only diffs.

M = 1 << 16
FNV_OFF = 1469598103934665603
FNV_PRM = 1099511628211
MASK64 = (1 << 64) - 1


def weak_init(data, i, L):
    a = b = 0
    for k in range(i, i + L):
        a = (a + data[k]) % M
        b = (b + (L - (k - i)) * data[k]) % M
    return a, b


def weak_roll(a, b, data, i, L):  # window [i,i+L-1] -> [i+1,i+L]
    a = (a - data[i] + data[i + L]) % M
    b = (b - L * data[i] + a) % M
    return a, b


def strong(block):
    h = FNV_OFF
    for byte in block:
        h = ((h ^ byte) * FNV_PRM) & MASK64
    return h


def diff(old, new, L):
    table = {}
    nblocks = len(old) // L
    for idx in range(nblocks):
        a, b = weak_init(old, idx * L, L)
        w = (b << 16) | a
        table.setdefault(w, {})[strong(old[idx * L:(idx + 1) * L])] = idx
    tokens, lit, i, n = [], bytearray(), 0, len(new)
    a = b = 0
    have = False
    while i < n:
        if i + L <= n:
            if not have:
                a, b = weak_init(new, i, L)
                have = True
            w = (b << 16) | a
            if w in table and strong(new[i:i + L]) in table[w]:
                if lit:
                    tokens.append(("LIT", bytes(lit)))
                    lit = bytearray()
                tokens.append(("COPY", table[w][strong(new[i:i + L])]))
                i += L
                have = False
                continue
            lit.append(new[i])
            if i + L <= n - 1:
                a, b = weak_roll(a, b, new, i, L)
            else:
                have = False
            i += 1
        else:
            lit.append(new[i])
            i += 1
    if lit:
        tokens.append(("LIT", bytes(lit)))
    return tokens


def patch(old, tokens, L):
    out = bytearray()
    for kind, val in tokens:
        if kind == "COPY":
            out += old[val * L:(val + 1) * L]
        else:
            out += val
    return bytes(out)


if __name__ == "__main__":
    L = 5
    old = b"the quick brown fox jumps over"
    new = b"the quick brown cat jumps over"
    tokens = diff(old, new, L)
    rebuilt = patch(old, tokens, L)
    lit_bytes = sum(len(v) for k, v in tokens if k == "LIT")
    print(rebuilt == new)                  # True
    print(lit_bytes, "of", len(new))       # literal bytes transferred
