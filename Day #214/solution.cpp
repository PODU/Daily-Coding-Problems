// Day 214: Longest consecutive run of 1s in binary representation.
// Approach: Brian Kernighan-style n &= (n<<1) collapses runs; count iterations. Time O(longest run), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int longestRun(unsigned long long n) {
    int count = 0;
    while (n) {
        n &= (n << 1);
        count++;
    }
    return count;
}

int main() {
    cout << longestRun(156) << endl; // 156 = 10011100 -> 3
    return 0;
}
