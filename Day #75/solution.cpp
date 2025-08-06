// Longest strictly increasing subsequence via patience sorting (tails array + lower_bound).
// Time O(n log n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

int lengthOfLIS(const vector<int>& a) {
    vector<int> tails;
    for (int x : a) {
        auto it = lower_bound(tails.begin(), tails.end(), x);
        if (it == tails.end()) tails.push_back(x);
        else *it = x;
    }
    return (int)tails.size();
}

int main() {
    vector<int> a = {0,8,4,12,2,10,6,14,1,9,5,13,3,11,7,15};
    cout << lengthOfLIS(a) << "\n";
    return 0;
}
