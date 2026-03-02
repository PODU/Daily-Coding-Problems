// De Bruijn B(k,n) via FKM (Lyndon word) algorithm -> lexicographically smallest. O(k^n).
#include <bits/stdc++.h>
using namespace std;

int K, N;            // K = alphabet size, N = substring length
vector<int> a;
string seq;

void db(int t, int p) {
    if (t > N) {
        if (N % p == 0)
            for (int i = 1; i <= p; i++) seq += char('0' + a[i]);
    } else {
        a[t] = a[t - p];
        db(t + 1, p);
        for (int j = a[t - p] + 1; j < K; j++) {
            a[t] = j;
            db(t + 1, t);
        }
    }
}

string deBruijn(int k, int n) {
    K = k; N = n;
    a.assign(k * n + 1, 0);
    seq.clear();
    db(1, 1);
    return seq;
}

int main() {
    // C = {0,1} -> alphabet size 2, substring length 3
    cout << deBruijn(2, 3) << "\n";
    return 0;
}
