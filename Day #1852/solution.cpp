// Day 1852: Longest Increasing Subsequence (strict).
// Patience sorting: keep tails[]; binary-search insertion point. O(n log n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

int lis(const vector<int>& a) {
    vector<int> tails;
    for (int x : a) {
        auto it = lower_bound(tails.begin(), tails.end(), x);
        if (it == tails.end()) tails.push_back(x);
        else *it = x;
    }
    return (int)tails.size();
}

int main() {
    vector<int> a = {10, 9, 2, 5, 3, 7, 101, 18};
    cout << lis(a) << "\n"; // 4 ([2,3,7,101] or [2,3,7,18])
    return 0;
}
