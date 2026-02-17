// First N regular (Hamming) numbers via 3-pointer merge of {2,3,5} multiples. Time O(N), Space O(N).
#include <bits/stdc++.h>
using namespace std;

vector<long long> regular(int n) {
    vector<long long> h(n);
    h[0] = 1;
    int i2 = 0, i3 = 0, i5 = 0;
    for (int i = 1; i < n; i++) {
        long long n2 = h[i2] * 2, n3 = h[i3] * 3, n5 = h[i5] * 5;
        long long m = min({n2, n3, n5});
        h[i] = m;
        if (m == n2) i2++;
        if (m == n3) i3++;
        if (m == n5) i5++;
    }
    return h;
}

int main() {
    vector<long long> res = regular(10);
    cout << "[";
    for (size_t i = 0; i < res.size(); i++) cout << res[i] << (i + 1 < res.size() ? ", " : "");
    cout << "]\n";
}
