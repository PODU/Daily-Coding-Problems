# Day 381: Hex string -> bytes -> standard Base64 (with '=' padding).
# Time: O(n), Space: O(n).  Note: canonical Base64 of "deadbeef" is "3q2+7w==".
B64 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"


def hex_to_base64(h):
    b = bytes(int(h[i:i+2], 16) for i in range(0, len(h), 2))
    out = []
    for i in range(0, len(b), 3):
        chunk = b[i:i+3]
        rem = len(chunk)
        n = chunk[0] << 16
        if rem > 1:
            n |= chunk[1] << 8
        if rem > 2:
            n |= chunk[2]
        out.append(B64[(n >> 18) & 63])
        out.append(B64[(n >> 12) & 63])
        out.append(B64[(n >> 6) & 63] if rem > 1 else "=")
        out.append(B64[n & 63] if rem > 2 else "=")
    return "".join(out)


if __name__ == "__main__":
    print(hex_to_base64("deadbeef"))
