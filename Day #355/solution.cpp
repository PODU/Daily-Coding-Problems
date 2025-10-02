// Round floats to ints keeping sum == round(total) with minimal total abs diff.
// Floor all; round up (T-F) elements with largest fractional parts (cheapest to push up). O(N log N) time, O(N) space.
#include <iostream>
#include <vector>
#include <cmath>
#include <algorithm>
using namespace std;

int main() {
    vector<double> x = {1.3, 2.3, 4.4};
    int n = x.size();
    vector<long long> y(n);
    double total = 0;
    long long floorSum = 0;
    for (int i = 0; i < n; i++) {
        double f = floor(x[i]);
        y[i] = (long long)f;
        floorSum += y[i];
        total += x[i];
    }
    long long target = llround(total);
    long long need = target - floorSum;

    // indices sorted by fractional part descending
    vector<int> idx(n);
    for (int i = 0; i < n; i++) idx[i] = i;
    sort(idx.begin(), idx.end(), [&](int a, int b) {
        return (x[a] - floor(x[a])) > (x[b] - floor(x[b]));
    });

    for (int k = 0; k < n && need > 0; k++) {
        int i = idx[k];
        if (x[i] - floor(x[i]) > 0) { y[i] += 1; need--; }
    }

    cout << "[";
    for (int i = 0; i < n; i++) {
        cout << y[i];
        if (i + 1 < n) cout << ", ";
    }
    cout << "]" << endl;
    return 0;
}
