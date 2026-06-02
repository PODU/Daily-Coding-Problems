// Total set bits from 1..N. Per-bit counting: O(log N) time, O(1) space.
// For each bit i: full cycles contribute 2^i ones each, plus remainder.
#include <iostream>
using namespace std;

long long countSetBits(long long n) {
    long long total = 0;
    for (long long i = 0; (1LL << i) <= n; ++i) {
        long long block = 1LL << (i + 1);
        long long ones = (n + 1) / block * (1LL << i);
        long long rem = (n + 1) % block - (1LL << i);
        if (rem > 0) ones += rem;
        total += ones;
    }
    return total;
}

int main() {
    cout << "N=5  -> " << countSetBits(5) << endl;   // 7
    cout << "N=16 -> " << countSetBits(16) << endl;  // 33
    return 0;
}
