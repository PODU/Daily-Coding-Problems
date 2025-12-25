// Day 801: nth sevenish number = sum of unique powers of 7.
// Bits of n select which powers of 7 to add (base-7 analog of binary).
// Time O(log n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

long long sevenish(int n) {
    long long result = 0, power = 1; // 7^0
    while (n) {
        if (n & 1) result += power;
        power *= 7;
        n >>= 1;
    }
    return result;
}

int main() {
    for (int i = 1; i <= 5; i++)
        cout << sevenish(i) << (i < 5 ? " " : "\n"); // 1 7 8 49 50
    return 0;
}
