// Day 221: nth "sevenish" number (sum of distinct powers of 7).
// Approach: bits of n select which powers of 7 to include (bijection with binary). Time O(log n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

long long sevenish(long long n) {
    long long result = 0, power = 1; // 7^0
    while (n > 0) {
        if (n & 1) result += power;
        power *= 7;
        n >>= 1;
    }
    return result;
}

int main() {
    for (int i = 1; i <= 5; i++) cout << sevenish(i) << (i < 5 ? " " : "\n"); // 1 7 8 49 50
    cout << sevenish(4) << endl; // 49
    return 0;
}
