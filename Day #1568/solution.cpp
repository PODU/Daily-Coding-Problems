// Longest consecutive run of 1s using n &= (n<<1) bit trick. Time O(run length), space O(1).
#include <bits/stdc++.h>
using namespace std;

int longestRun(unsigned long long n) {
    int count = 0;
    while (n) { n &= (n << 1); count++; }
    return count;
}

int main() {
    cout << longestRun(156) << endl; // 10011100 -> 3
    return 0;
}
