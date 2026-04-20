// Josephus problem via iterative recurrence j(i)=(j(i-1)+k)%i, answer j(N)+1. O(N) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

int josephus(int n, int k) {
    int res = 0;
    for (int i = 2; i <= n; i++) res = (res + k) % i;
    return res + 1;
}

int main() {
    cout << josephus(5, 2) << "\n";
    return 0;
}
