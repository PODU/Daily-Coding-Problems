// Kaprekar's routine: repeatedly subtract ascending-digit from descending-digit
// (4-digit, zero-padded) until 6174; count steps. Time: O(7) iters, Space: O(1).
#include <bits/stdc++.h>
using namespace std;

int kaprekar(int n) {
    int steps = 0;
    while (n != 6174) {
        char buf[5];
        sprintf(buf, "%04d", n);
        string d(buf);
        string asc = d, desc = d;
        sort(asc.begin(), asc.end());
        sort(desc.rbegin(), desc.rend());
        n = stoi(desc) - stoi(asc);
        steps++;
    }
    return steps;
}

int main() {
    cout << kaprekar(1234) << endl;
    return 0;
}
