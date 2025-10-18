// Day 454: Longest Increasing Subsequence length.
// Patience sorting with binary search. Time O(n log n), Space O(n).
#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

int lis(const vector<int>& a) {
    vector<int> tails; // tails[k] = smallest tail of an increasing subseq of length k+1
    for (int x : a) {
        auto it = lower_bound(tails.begin(), tails.end(), x);
        if (it == tails.end()) tails.push_back(x);
        else *it = x;
    }
    return (int)tails.size();
}

int main() {
    vector<int> a = {10, 9, 2, 5, 3, 7, 101, 18};
    cout << lis(a) << endl; // 4  -> [2,3,7,101]
    return 0;
}
