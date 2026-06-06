// Sevenish: write n in binary; each set bit k contributes 7^k. Time O(log n), Space O(1).
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
    for (int n = 1; n <= 5; ++n)
        cout << sevenish(n) << (n < 5 ? " " : "\n");
    return 0;
}
