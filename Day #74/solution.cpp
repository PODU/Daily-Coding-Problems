// Count occurrences of X in N×N table: for each row i (1..N), X appears iff i|X and X/i in [1..N].
// Time O(N), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int countX(long long N, long long X) {
    int cnt = 0;
    for (long long i = 1; i <= N; ++i)
        if (X % i == 0 && X / i >= 1 && X / i <= N) ++cnt;
    return cnt;
}

int main() {
    cout << countX(6, 12) << "\n";
    return 0;
}
