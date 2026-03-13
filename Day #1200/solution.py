# Day 1200: Reverse all 32 bits by shifting LSB of input into LSB-first of result (mask to 32 bits).
# Time: O(1) (32 steps); Space: O(1).
def reverse_bits(x):
    r = 0
    for _ in range(32):
        r = (r << 1) | (x & 1)
        x >>= 1
    return r & 0xFFFFFFFF

def to_grouped(x):
    bits = format(x & 0xFFFFFFFF, "032b")
    return " ".join(bits[i:i+4] for i in range(0, 32, 4))

if __name__ == "__main__":
    inp = 0xF0F0F0F0
    print("Input: ", to_grouped(inp))
    print(to_grouped(reverse_bits(inp)))
