# Day 826: rsync-style file sync over a low-bandwidth link.
# Receiver signs old file's fixed-size blocks (weak rolling Adler checksum +
# strong FNV-1a hash); sender rolls a window over the new file to find matching
# blocks, emitting COPY(block) / LITERAL(bytes) tokens; receiver rebuilds.
# Time O(n) average (rolling hash), Space O(n).

MOD = 1 << 16
L = 4  # block size


def weak_block(b):
    a = sum(b) % MOD
    s = sum((len(b) - k) * b[k] for k in range(len(b))) % MOD
    return a, s, (a + MOD * s)


def strong(b):
    h = 1469598103934665603
    for x in b:
        h ^= x
        h = (h * 1099511628211) & 0xFFFFFFFFFFFFFFFF
    return h


def signature(old):
    blocks = [old[i:i + L] for i in range(0, len(old), L)]
    table = {}
    for idx, blk in enumerate(blocks):
        if len(blk) == L:
            _, _, w = weak_block(blk)
            table.setdefault(w, []).append((strong(blk), idx))
    return blocks, table


def diff(new, blocks, table):
    """Sender: produce delta of ('copy', idx) and ('lit', bytes) tokens."""
    delta, lit = [], bytearray()
    n = len(new)
    i = 0
    a = s = cs = 0
    have_window = False
    while i < n:
        if i + L <= n:
            if not have_window:
                a, s, cs = weak_block(new[i:i + L])
                have_window = True
            if cs in table:
                hh = strong(new[i:i + L])
                hit = next((idx for (sh, idx) in table[cs]
                            if sh == hh and new[i:i + L] == blocks[idx]), -1)
                if hit >= 0:
                    if lit:
                        delta.append(('lit', bytes(lit)))
                        lit = bytearray()
                    delta.append(('copy', hit))
                    i += L
                    have_window = False
                    continue
            # no match: emit one literal byte, roll window forward by 1
            lit.append(new[i])
            if i + L < n:
                a = (a - new[i] + new[i + L]) % MOD
                s = (s - L * new[i] + a) % MOD
                cs = a + MOD * s
            i += 1
        else:
            lit.append(new[i])
            i += 1
    if lit:
        delta.append(('lit', bytes(lit)))
    return delta


def reconstruct(blocks, delta):
    out = bytearray()
    for tok in delta:
        if tok[0] == 'copy':
            out += blocks[tok[1]]
        else:
            out += tok[1]
    return bytes(out)


if __name__ == "__main__":
    old = b"the quick brown fox jumps"
    new = b"the quick red fox leaps over"
    blocks, table = signature(old)
    delta = diff(new, blocks, table)
    rebuilt = reconstruct(blocks, delta)
    copies = sum(1 for t in delta if t[0] == 'copy')
    print(rebuilt.decode())
    print("reconstruction OK:", rebuilt == new)
    print("blocks reused (copy tokens):", copies)
