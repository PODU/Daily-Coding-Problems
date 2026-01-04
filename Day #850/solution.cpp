// Day 850: De Bruijn sequence B(k_alphabet, n) via the FKM (Lyndon-word) algorithm.
// Generates the lexicographically smallest cyclic sequence containing every length-n string once.
// O(k^n) time = length of output.
#include <bits/stdc++.h>
using namespace std;

string deBruijn(int k /*alphabet size*/, int n /*substring length*/, const string& alphabet){
    vector<int> a(k * n, 0);
    string seq;
    function<void(int,int)> db = [&](int t, int p){
        if(t > n){
            if(n % p == 0)
                for(int j = 1; j <= p; ++j) seq.push_back(alphabet[a[j]]);
        } else {
            a[t] = a[t-p];
            db(t+1, p);
            for(int j = a[t-p] + 1; j < k; ++j){ a[t] = j; db(t+1, t); }
        }
    };
    db(1, 1);
    return seq;
}

int main(){
    // C = {0,1}, k(length) = 3  => alphabet size 2, n = 3
    cout << deBruijn(2, 3, "01") << "\n"; // 00010111
    return 0;
}
