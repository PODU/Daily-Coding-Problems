// Prefix-sum array P (P[k]=sum of first k elems); sum(i,j)=P[j]-P[i]. Preprocess O(n), query O(1) time/space per query.
#include <iostream>
#include <vector>
using namespace std;

struct RangeSum {
    vector<long long> P;
    RangeSum(const vector<int>& L) {
        P.assign(L.size() + 1, 0);
        for (size_t k = 0; k < L.size(); ++k) P[k + 1] = P[k] + L[k];
    }
    long long sum(int i, int j) { return P[j] - P[i]; }
};

int main() {
    vector<int> L = {1, 2, 3, 4, 5};
    RangeSum rs(L);
    cout << rs.sum(1, 3) << "\n";
    return 0;
}
