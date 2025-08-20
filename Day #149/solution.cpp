// Range sum via prefix sums: O(n) preprocessing, O(1) per query. sum(i,j) = pre[j]-pre[i].
#include <iostream>
#include <vector>
using namespace std;

class RangeSum {
    vector<long long> pre;
public:
    RangeSum(const vector<long long>& L) {
        pre.assign(L.size() + 1, 0);
        for (size_t k = 0; k < L.size(); k++) pre[k + 1] = pre[k] + L[k];
    }
    long long sum(int i, int j) { return pre[j] - pre[i]; }
};

int main() {
    vector<long long> L = {1, 2, 3, 4, 5};
    RangeSum rs(L);
    cout << rs.sum(1, 3) << endl;
    return 0;
}
