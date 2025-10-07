# Day 385: Brute-force all 256 single-byte keys; score by letters/spaces to pick plaintext.
# Time: O(256 * n), Space: O(n).

def score(s):
    if any(c < 32 or c > 126 for c in s):
        return -1
    return sum(1 for c in s if chr(c).isalpha() or c == 32)


def decrypt(hex_str):
    data = bytes.fromhex(hex_str)
    best, best_score = "", -1
    for k in range(256):
        cand = bytes(b ^ k for b in data)
        sc = score(cand)
        if sc > best_score:
            best_score, best = sc, cand.decode("latin-1")
    return best


if __name__ == "__main__":
    h = "7a575e5e5d12455d405e561254405d5f1276535b5e4b12715d565b5c551262405d505e575f"
    print(decrypt(h))
