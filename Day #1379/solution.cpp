// Max of two without if/comparison: subtract the masked difference using the sign bit.
// max(a,b) = a - ((a-b) & ((a-b)>>63)). Time O(1), Space O(1).
#include <bits/stdc++.h>
using namespace std;

long long maxNoBranch(long long a, long long b) {
    long long diff = a - b;
    return a - (diff & (diff >> 63));
}

int main() {
    cout << maxNoBranch(3, 7) << "\n";   // 7
    cout << maxNoBranch(10, -5) << "\n"; // 10
    return 0;
}
