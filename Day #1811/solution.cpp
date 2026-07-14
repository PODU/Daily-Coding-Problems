// Longest strictly increasing subsequence via patience sorting + binary search.
// Time: O(n log n). Space: O(n).
#include <bits/stdc++.h>
using namespace std;

int lis(const vector<int>& a) {
    vector<int> tails; // tails[i] = smallest tail of an increasing subseq of length i+1
    for (int x : a) {
        auto it = lower_bound(tails.begin(), tails.end(), x);
        if (it == tails.end()) tails.push_back(x);
        else *it = x;
    }
    return tails.size();
}

int main() {
    vector<int> a = {0,8,4,12,2,10,6,14,1,9,5,13,3,11,7,15};
    cout << lis(a) << "\n"; // 6
    return 0;
}
