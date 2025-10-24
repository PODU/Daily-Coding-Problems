// Day 483: Josephus problem.
// Iterative recurrence f(i)=(f(i-1)+k)%i in O(n) time, O(1) space.
// Special O(log n) closed form when k=2.
#include <bits/stdc++.h>
using namespace std;

int josephus(int n, int k) {
    int result = 0; // 0-indexed survivor among 1 person
    for (int i = 2; i <= n; ++i)
        result = (result + k) % i;
    return result + 1; // convert to 1-indexed
}

// O(log n) special case for k == 2: survivor = 2*(n - 2^floor(log2 n)) + 1
int josephusK2(int n) {
    int p = 1;
    while (p * 2 <= n) p *= 2; // largest power of two <= n
    return 2 * (n - p) + 1;
}

int main() {
    int n = 5, k = 2;
    cout << josephus(n, k) << "\n"; // 3
    // sanity: O(log n) form agrees for k=2
    // cout << josephusK2(n) << "\n";
    return 0;
}
