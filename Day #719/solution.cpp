// Day 719: Convert 1-based column number to spreadsheet id (bijective base-26).
// Repeatedly take (n-1)%26 then n=(n-1)/26. Time O(log n).
#include <bits/stdc++.h>
using namespace std;

string colId(long long n) {
    string s;
    while (n > 0) {
        --n;
        s += char('A' + n % 26);
        n /= 26;
    }
    reverse(s.begin(), s.end());
    return s;
}

int main() {
    cout << "\"" << colId(1) << "\"" << endl;
    cout << "\"" << colId(27) << "\"" << endl;
    return 0;
}
