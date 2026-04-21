# Day 1401: rsync-style delta sync: receiver indexes fixed-size blocks of its old file by
# a rolling (Adler-like) weak hash + exact block as strong check. Sender slides a
# rolling window over the new file, emitting COPY(block) for matches else literal
# bytes, so only differences cross the wire. Time O(N) avg, Space O(old/B).

B = 4


def weak(block):
    a = b = 0
    n = len(block)
    for i, c in enumerate(block):
        a = (a + c) & 0xFFFF
        b = (b + (n - i) * c) & 0xFFFF
    return a, b


def make_delta(old, new):
    old, new = old.encode(), new.encode()
    table = {}
    for idx in range(0, len(old) - B + 1, B):
        blk = old[idx:idx + B]
        a, b = weak(blk)
        table.setdefault((b << 16) | a, []).append((idx, blk))
    delta = []
    i, n = 0, len(new)
    a = b = 0
    if n >= B:
        a, b = weak(new[:B])
    while i < n:
        if i + B <= n:
            h = (b << 16) | a
            match = None
            for idx, blk in table.get(h, []):
                if blk == new[i:i + B]:
                    match = idx
                    break
            if match is not None:
                delta.append(("copy", match))
                i += B
                if i + B <= n:
                    a, b = weak(new[i:i + B])
                continue
        delta.append(("lit", new[i]))
        if i + B < n:
            out, inb = new[i], new[i + B]
            a = (a - out + inb) & 0xFFFF
            b = (b - B * out + a) & 0xFFFF
        i += 1
    return delta


def rebuild(old, delta):
    old = old.encode()
    out = bytearray()
    for t in delta:
        if t[0] == "copy":
            out += old[t[1]:t[1] + B]
        else:
            out.append(t[1])
    return out.decode()


if __name__ == "__main__":
    old = "the quick brown fox jumps over the lazy dog"
    new = "the quick brown cat jumps over the lazy dog"
    delta = make_delta(old, new)
    copies = sum(1 for t in delta if t[0] == "copy")
    lits = sum(1 for t in delta if t[0] == "lit")
    print("Reconstructed:", rebuild(old, delta))
    print("Match:", str(rebuild(old, delta) == new).lower())
    print("copied blocks:", copies, "literal bytes:", lits)
