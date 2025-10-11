// Day 410: Round floats so rounded sum is preserved with minimal total abs error.
// Floor all, then round up the R = round(sum)-sum(floors) elements with largest fractions. O(n log n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

vector<long long> roundToSum(const vector<double>& x) {
    int n = x.size();
    vector<long long> y(n);
    double total = 0;
    long long floorSum = 0;
    for (int i = 0; i < n; i++) {
        y[i] = (long long)floor(x[i]);
        floorSum += y[i];
        total += x[i];
    }
    long long target = llround(total);
    long long R = target - floorSum; // number of elements to round up
    vector<int> idx(n);
    iota(idx.begin(), idx.end(), 0);
    // largest fractional parts first
    sort(idx.begin(), idx.end(), [&](int a, int b) {
        return (x[a] - floor(x[a])) > (x[b] - floor(x[b]));
    });
    for (long long i = 0; i < R && i < n; i++) y[idx[i]] += 1;
    return y;
}

int main() {
    vector<double> x = {1.3, 2.3, 4.4};
    vector<long long> y = roundToSum(x);
    cout << "[";
    for (size_t i = 0; i < y.size(); i++) {
        cout << y[i];
        if (i + 1 < y.size()) cout << ", ";
    }
    cout << "]" << endl;
    return 0;
}
