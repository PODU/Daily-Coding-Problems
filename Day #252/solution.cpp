// Egyptian fraction via greedy: repeatedly take ceil(b/a) as next unit denominator.
// Time: O(number of terms) iterations; Space: O(terms). a/b proper (a<b).
#include <bits/stdc++.h>
using namespace std;

vector<long long> egyptian(long long a, long long b) {
    vector<long long> denoms;
    while (a != 0) {
        long long d = (b + a - 1) / a; // ceil(b/a)
        denoms.push_back(d);
        a = a * d - b;
        b = b * d;
    }
    return denoms;
}

int main() {
    auto denoms = egyptian(4, 13);
    for (size_t i = 0; i < denoms.size(); i++) {
        cout << "1 / " << denoms[i];
        if (i + 1 < denoms.size()) cout << " + ";
    }
    cout << "\n";
    return 0;
}
