// Reconstruct a permutation of [0..N] from +/- signs. Two-pointer, O(N).
// '+' takes the current low, '-' takes the current high; produces a valid order.
#include <bits/stdc++.h>
using namespace std;

vector<int> reconstruct(vector<char>& signs){ // signs: '+'/'-' constraints (None dropped)
    int L = signs.size() + 1, N = L - 1;
    vector<int> res(L);
    int low = 0, high = N;
    for(int j = 0; j < (int)signs.size(); j++){
        if(signs[j] == '+') res[j] = low++;
        else res[j] = high--;
    }
    res[L-1] = low;
    return res;
}

int main(){
    // input [None, +, +, -, +] -> constraints (+,+,-,+)
    vector<char> signs = {'+','+','-','+'};
    vector<int> r = reconstruct(signs);
    cout << "[";
    for(size_t i = 0; i < r.size(); i++) cout << r[i] << (i+1<r.size() ? ", " : "");
    cout << "]\n"; // a valid reconstruction, e.g. [0, 1, 4, 2, 3]
    return 0;
}
