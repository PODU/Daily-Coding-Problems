// Day 776: Josephus problem. General O(N) recurrence j=(j+k)%i.
// For k=2 an O(log N) closed form is also given. Returns 1-indexed survivor.
#include <bits/stdc++.h>
using namespace std;

int josephus(int n, int k) {
    int r = 0;                       // 0-indexed survivor
    for (int i = 2; i <= n; i++) r = (r + k) % i;
    return r + 1;                    // 1-indexed
}

int josephusK2(int n) {              // O(log N): 2*(n - 2^floor(log2 n)) + 1
    int p = 1;
    while (p * 2 <= n) p *= 2;
    return 2 * (n - p) + 1;
}

int main() {
    cout << josephus(5, 2) << endl;   // 3
    cout << josephusK2(5) << endl;    // 3
    return 0;
}
