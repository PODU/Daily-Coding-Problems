// Day 212: Spreadsheet column number -> label (bijective base-26).
// Approach: repeatedly take (n-1)%26 for the digit, then n=(n-1)/26. Time O(log n), Space O(log n).
#include <bits/stdc++.h>
using namespace std;

string columnLabel(long long n) {
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
    cout << "\"" << columnLabel(1) << "\"" << endl;
    cout << "\"" << columnLabel(27) << "\"" << endl;
    return 0;
}
