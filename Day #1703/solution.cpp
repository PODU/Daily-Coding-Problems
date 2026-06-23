// Prefix sums: precompute prefix[k]=sum(L[0:k]); sum(i,j)=prefix[j]-prefix[i].
// Preprocess O(n), query O(1), Space O(n).
#include <bits/stdc++.h>
using namespace std;

struct RangeSum {
    vector<long long> prefix;
    RangeSum(const vector<int>& L) {
        prefix.resize(L.size() + 1, 0);
        for (size_t k = 0; k < L.size(); k++) prefix[k + 1] = prefix[k] + L[k];
    }
    long long sum(int i, int j) { return prefix[j] - prefix[i]; }
};

int main() {
    vector<int> L = {1, 2, 3, 4, 5};
    RangeSum rs(L);
    cout << rs.sum(1, 3) << "\n";
    return 0;
}
