// Max of two numbers without if-else/branch/comparison via bit manipulation.
// d=a-b; mask = d>>63 (arithmetic sign extend); max = a - (d & mask). Time O(1), Space O(1).
#include <iostream>
#include <cstdint>
using namespace std;

long long maxNoBranch(long long a, long long b) {
    long long d = a - b;
    long long mask = d >> 63; // all 1s if d<0, else 0
    return a - (d & mask);
}

int main() {
    cout << maxNoBranch(3, 7) << "\n";
    cout << maxNoBranch(10, -4) << "\n";
    return 0;
}
