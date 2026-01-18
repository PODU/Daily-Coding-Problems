// Floor all; round up the `deficit` elements with largest fractional parts to match round(sum). O(n log n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

vector<int> roundToSum(const vector<double>& x) {
    int n = x.size();
    vector<int> y(n);
    double sum = 0;
    long long floorSum = 0;
    vector<pair<double,int>> frac(n);
    for (int i = 0; i < n; i++) {
        double f = floor(x[i]);
        y[i] = (int)f;
        floorSum += (long long)f;
        sum += x[i];
        frac[i] = {x[i] - f, i};
    }
    long long target = llround(sum);
    long long deficit = target - floorSum;
    // round up the `deficit` elements with largest fractional parts
    sort(frac.begin(), frac.end(), [](const pair<double,int>&a, const pair<double,int>&b){
        return a.first > b.first;
    });
    for (long long k = 0; k < deficit && k < n; k++) y[frac[k].second]++;
    return y;
}

int main() {
    vector<double> x = {1.3, 2.3, 4.4};
    vector<int> y = roundToSum(x);
    cout << "[";
    for (size_t i = 0; i < y.size(); i++) cout << y[i] << (i+1<y.size() ? ", " : "");
    cout << "]\n";
    return 0;
}
