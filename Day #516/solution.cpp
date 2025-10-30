// Sevenish: nth value = sum of 7^i for each set bit i of n. O(log n) per query.
#include <bits/stdc++.h>
using namespace std;

long long sevenish(long long n) {
    long long sum = 0, pow7 = 1;
    while (n > 0) {
        if (n & 1) sum += pow7;
        pow7 *= 7;
        n >>= 1;
    }
    return sum;
}

int main() {
    // First few sevenish numbers: 1, 7, 8, 49, ...
    for (int n = 1; n <= 4; n++) cout << sevenish(n) << (n < 4 ? ", " : "\n");
    return 0;
}
