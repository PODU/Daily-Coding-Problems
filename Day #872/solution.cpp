// Approximate median: take a small random sample (size s) and return its median.
// Rank lands in [N/4, 3N/4] w.h.p. Time O(s log s) = sub-linear, Space O(s).
#include <bits/stdc++.h>
using namespace std;

int approxMedian(const vector<int>& a, int sampleSize, mt19937& rng){
    int n = a.size();
    int s = min(sampleSize, n);
    vector<int> sample;
    sample.reserve(s);
    uniform_int_distribution<int> dist(0, n - 1);
    for(int i = 0; i < s; i++) sample.push_back(a[dist(rng)]);
    nth_element(sample.begin(), sample.begin() + s / 2, sample.end());
    return sample[s / 2];
}

int main(){
    mt19937 rng(42);
    vector<int> a(1000);
    for(int i = 0; i < 1000; i++) a[i] = i;          // values 0..999
    shuffle(a.begin(), a.end(), rng);                 // unordered
    int m = approxMedian(a, 51, rng);
    // rank check
    int rank = count_if(a.begin(), a.end(), [&](int x){ return x < m; });
    cout << "approx median = " << m << "\n";
    cout << "rank " << rank << " in [" << a.size()/4 << ", " << 3*a.size()/4 << "]: "
         << ((rank >= (int)a.size()/4 && rank <= 3*(int)a.size()/4) ? "true" : "false") << "\n";
    return 0;
}
