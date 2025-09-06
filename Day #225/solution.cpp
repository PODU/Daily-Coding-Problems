// Day 225: Josephus problem - position (1-indexed) of last survivor.
// Approach: general O(N) recurrence J(i)=(J(i-1)+k)%i. Bonus: k==2 closed form O(log N): 2*(n-2^floor(log2 n))+1.
#include <bits/stdc++.h>
using namespace std;

int josephus(int n, int k) {
    if (k == 2) {
        int p = 1;
        while (p * 2 <= n) p *= 2; // highest power of 2 <= n
        return 2 * (n - p) + 1;
    }
    int res = 0; // 0-indexed
    for (int i = 2; i <= n; i++) res = (res + k) % i;
    return res + 1;
}

int main() {
    cout << josephus(5, 2) << endl; // 3
    return 0;
}
