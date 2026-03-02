// Day 1142: nth sevenish number = sum of distinct powers of 7.
// Bits of n in binary select powers of 7 (bijection). Time O(log n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

long long sevenish(long long n) {
    long long result = 0, power = 1;
    while (n > 0) {
        if (n & 1) result += power;
        power *= 7;
        n >>= 1;
    }
    return result;
}

int main() {
    for (long long i = 1; i <= 5; ++i) cout << sevenish(i) << " "; // 1 7 8 49 50
    cout << "\n";
    return 0;
}
