// Weighted sampling: build cumulative prefix array, draw u in [0,1), upper_bound to pick. O(n) prep, O(log n) per sample.
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<int> numbers = {1, 2, 3, 4};
    vector<double> probs = {0.1, 0.5, 0.2, 0.2};
    int n = numbers.size();

    vector<double> cum(n);
    double acc = 0;
    for (int i = 0; i < n; i++) { acc += probs[i]; cum[i] = acc; }

    mt19937 rng(42);
    uniform_real_distribution<double> dist(0.0, 1.0);

    const int N = 100000;
    vector<long long> counts(n, 0);
    for (int s = 0; s < N; s++) {
        double u = dist(rng);
        int idx = lower_bound(cum.begin(), cum.end(), u) - cum.begin();
        if (idx >= n) idx = n - 1;
        counts[idx]++;
    }

    cout << fixed << setprecision(2);
    for (int i = 0; i < n; i++)
        cout << numbers[i] << ": " << (double)counts[i] / N << "\n";
    return 0;
}
