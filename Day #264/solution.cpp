// Day 264: De Bruijn sequence B(k, n) over a character set.
// Approach: Fredricksen-Kessler-Maiorana algorithm — concatenate Lyndon words
// whose length divides n, generated via Duval-style recursion.
// Time O(k^n) (size of the output), Space O(n).

#include <bits/stdc++.h>
using namespace std;

struct DeBruijn {
    vector<char> alphabet;
    int n, k;
    vector<int> a;
    string sequence;

    DeBruijn(const vector<char> &alpha, int n_) : alphabet(alpha), n(n_), k(alpha.size()) {
        a.assign(k * n, 0);
    }

    void db(int t, int p) {
        if (t > n) {
            if (n % p == 0)
                for (int i = 1; i <= p; i++) sequence += alphabet[a[i]];
        } else {
            a[t] = a[t - p];
            db(t + 1, p);
            for (int j = a[t - p] + 1; j < k; j++) {
                a[t] = j;
                db(t + 1, t);
            }
        }
    }

    string build() {
        db(1, 1);
        return sequence;
    }
};

int main() {
    // C = {0, 1}, k = 3
    DeBruijn d({'0', '1'}, 3);
    cout << d.build() << endl;
    return 0;
}
