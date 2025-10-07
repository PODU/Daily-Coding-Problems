# Day 382: Base64 -> bytes -> hex. Bit-accumulator decode (tolerates padding/whitespace).
# Time: O(n), Space: O(n).
B64 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
VAL = {c: i for i, c in enumerate(B64)}


def base64_to_hex(s):
    bits = 0
    nbits = 0
    out = []
    for c in s:
        if c not in VAL:
            continue
        bits = (bits << 6) | VAL[c]
        nbits += 6
        if nbits >= 8:
            nbits -= 8
            out.append((bits >> nbits) & 0xFF)
    return "".join(f"{b:02x}" for b in out)


if __name__ == "__main__":
    print(base64_to_hex("3q2+7w="))
