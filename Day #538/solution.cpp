// De Bruijn sequence via FKM (Lyndon-word/necklace) algorithm: emit Lyndon words whose
// length divides n, in order, giving lexicographically smallest sequence. Time O(k^n).
#include <bits/stdc++.h>
using namespace std;

string deBruijn(int k, int n) {
    vector<int> a(k * n + 1, 0);
    string seq;
    function<void(int,int)> db = [&](int t, int p) {
        if (t > n) {
            if (n % p == 0)
                for (int j = 1; j <= p; ++j) seq += char('0' + a[j]);
        } else {
            a[t] = a[t - p];
            db(t + 1, p);
            for (int j = a[t - p] + 1; j < k; ++j) {
                a[t] = j;
                db(t + 1, t);
            }
        }
    };
    db(1, 1);
    return seq;
}

int main() {
    cout << deBruijn(2, 3) << "\n"; // 00010111
    return 0;
}
