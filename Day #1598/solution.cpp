// Round floats preserving sum: floor all, then round up the d elements with
// largest fractional parts (d = round(sum) - sum(floors)). Time O(n log n).
#include <bits/stdc++.h>
using namespace std;

vector<long long> roundPreserve(const vector<double>& x) {
    int n = x.size();
    vector<long long> y(n);
    double sum = 0;
    long long floorSum = 0;
    for (int i = 0; i < n; i++) {
        long long f = (long long)floor(x[i]);
        y[i] = f;
        floorSum += f;
        sum += x[i];
    }
    long long target = llround(sum);
    long long d = target - floorSum;
    // indices sorted by descending fractional part
    vector<int> order(n);
    iota(order.begin(), order.end(), 0);
    sort(order.begin(), order.end(), [&](int a, int b) {
        return (x[a] - floor(x[a])) > (x[b] - floor(x[b]));
    });
    for (long long i = 0; i < d; i++) y[order[i]] += 1;
    return y;
}

int main() {
    vector<double> x = {1.3, 2.3, 4.4};
    vector<long long> y = roundPreserve(x);

    cout << "[";
    for (size_t i = 0; i < y.size(); i++) { if (i) cout << ", "; cout << y[i]; }
    cout << "]\n";

    double diff = 0;
    for (size_t i = 0; i < x.size(); i++) diff += fabs(x[i] - (double)y[i]);
    cout << "abs diff = " << fixed << setprecision(1) << diff << "\n";
    return 0;
}
