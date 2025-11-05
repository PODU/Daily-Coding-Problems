// Count occurrences of X in an N x N multiplication table.
// For each row i (1..N), X appears iff i divides X and X/i is in [1,N]. O(N) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

int countX(int N, long long X) {
    int cnt = 0;
    for (int i = 1; i <= N; ++i)
        if (X % i == 0) {
            long long q = X / i;
            if (q >= 1 && q <= N) ++cnt;
        }
    return cnt;
}

int main() {
    cout << countX(6, 12) << "\n"; // 4
    return 0;
}
