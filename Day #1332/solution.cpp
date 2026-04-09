// Day 1332: Round each x_i up or down so sum(Y)=round(sum X) while minimizing sum|x_i-y_i|.
// Floor everything, then round up the k elements with the largest fractional parts (k=target-sum of floors). O(n log n).
#include <bits/stdc++.h>
using namespace std;

vector<long long> roundPreserveSum(const vector<double>& x) {
    int n = x.size();
    double s = 0;
    for (double v : x) s += v;
    long long target = llround(s);
    vector<long long> y(n);
    long long base = 0;
    vector<pair<double,int>> fracs;
    for (int i = 0; i < n; i++) {
        long long f = (long long)floor(x[i]);
        y[i] = f; base += f;
        fracs.push_back({x[i] - f, i});
    }
    long long k = target - base; // number to round up
    sort(fracs.begin(), fracs.end(), [](auto& a, auto& b){ return a.first > b.first; });
    for (int i = 0; i < k; i++) y[fracs[i].second]++;
    return y;
}

int main() {
    vector<double> x = {1.3, 2.3, 4.4};
    auto y = roundPreserveSum(x);
    cout << "[";
    for (size_t i = 0; i < y.size(); i++) cout << y[i] << (i + 1 < y.size() ? ", " : "");
    cout << "]" << endl; // [1, 2, 5]
    return 0;
}
