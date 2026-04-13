// Longest run of consecutive 1s in binary. Bit trick: n &= (n<<1) collapses runs; iterations = longest run. O(run) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

int longestRun(unsigned int n) {
    int count = 0;
    while (n) {
        n &= (n << 1);
        count++;
    }
    return count;
}

int main() {
    cout << longestRun(156) << endl;
    return 0;
}
