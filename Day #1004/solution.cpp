// Day 1004: Range sum query sum(i, j) = L[i:j].
// Pre-process a prefix-sum array (O(N)); each query is prefix[j]-prefix[i] in O(1).
#include <bits/stdc++.h>
using namespace std;

struct RangeSum {
    vector<long long> prefix;
    RangeSum(const vector<long long>& L) : prefix(L.size() + 1, 0) {
        for (size_t i = 0; i < L.size(); ++i) prefix[i + 1] = prefix[i] + L[i];
    }
    long long sum(int i, int j) const { return prefix[j] - prefix[i]; }
};

int main() {
    RangeSum rs({1, 2, 3, 4, 5});
    cout << rs.sum(1, 3) << "\n"; // 5
    return 0;
}
