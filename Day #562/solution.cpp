// Product of array except self without division.
// Prefix * suffix products in two passes. Time O(N), Space O(1) extra (besides output).
#include <iostream>
#include <vector>
using namespace std;

vector<long long> productExceptSelf(const vector<long long>& a) {
    int n = a.size();
    vector<long long> res(n, 1);
    long long prefix = 1;
    for (int i = 0; i < n; ++i) { res[i] = prefix; prefix *= a[i]; }
    long long suffix = 1;
    for (int i = n - 1; i >= 0; --i) { res[i] *= suffix; suffix *= a[i]; }
    return res;
}

void printVec(const vector<long long>& v) {
    cout << "[";
    for (size_t i = 0; i < v.size(); ++i) { if (i) cout << ", "; cout << v[i]; }
    cout << "]\n";
}

int main() {
    printVec(productExceptSelf({1, 2, 3, 4, 5}));
    printVec(productExceptSelf({3, 2, 1}));
    return 0;
}
