// De Bruijn sequence B(k_alphabet, n) via the FKM/Lyndon-word recursion.
// Concatenating Lyndon words whose length divides n. Time: O(k^n). Space: O(k^n).
#include <bits/stdc++.h>
using namespace std;

string deBruijn(const vector<char>& C, int n) {
    int k = C.size();
    vector<int> a(k * n, 0);
    vector<int> seq;
    function<void(int,int)> db = [&](int t, int p) {
        if (t > n) {
            if (n % p == 0)
                for (int j = 1; j <= p; j++) seq.push_back(a[j]);
        } else {
            a[t] = a[t - p];
            db(t + 1, p);
            for (int j = a[t - p] + 1; j < k; j++) {
                a[t] = j;
                db(t + 1, t);
            }
        }
    };
    db(1, 1);
    string res;
    for (int idx : seq) res += C[idx];
    return res;
}

int main() {
    vector<char> C = {'0', '1'};
    cout << deBruijn(C, 3) << "\n"; // 00010111
    return 0;
}
