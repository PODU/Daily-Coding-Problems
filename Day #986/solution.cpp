// Longest consecutive run of 1-bits via the bit trick: n &= (n>>1) shrinks every
// run by one each step; iterations until n==0 equals the longest run length.
// Time: O(longest run), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

int longestRun(unsigned int n) {
    int count = 0;
    while (n) {
        count++;
        n &= (n >> 1);
    }
    return count;
}

int main() {
    cout << longestRun(156) << "\n"; // 156 = 10011100 -> 3
    return 0;
}
