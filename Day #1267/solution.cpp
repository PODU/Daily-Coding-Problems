// Day 1267: Range sum query with preprocessing.
// Prefix-sum array: O(n) preprocess, O(1) per sum(i, j) query.
#include <bits/stdc++.h>
using namespace std;

struct RangeSum {
    vector<long long> prefix;
    RangeSum(const vector<int>& L) {
        prefix.assign(L.size() + 1, 0);
        for (size_t i = 0; i < L.size(); ++i) prefix[i + 1] = prefix[i] + L[i];
    }
    long long sum(int i, int j) { return prefix[j] - prefix[i]; } // L[i:j]
};

int main() {
    RangeSum rs({1, 2, 3, 4, 5});
    cout << rs.sum(1, 3) << endl;
    return 0;
}
