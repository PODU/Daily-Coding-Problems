// Power of four: N>0 && single set bit (N&(N-1))==0 && bit at even position (N & 0x55555555). O(1).
#include <bits/stdc++.h>
using namespace std;

bool isPowerOfFour(unsigned int n) {
    return n > 0 && (n & (n - 1)) == 0 && (n & 0x55555555u) != 0;
}

int main() {
    int tests[] = {16, 8, 1, 64, 5};
    for (int t : tests)
        cout << t << " -> " << (isPowerOfFour(t) ? "true" : "false") << "\n";
    return 0;
}
