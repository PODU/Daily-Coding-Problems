// Day 1330: Column number -> spreadsheet label (bijective base-26).
// Repeatedly take (n-1)%26 for the next letter, then n=(n-1)/26. O(log n) time.
#include <bits/stdc++.h>
using namespace std;

string columnTitle(long long n) {
    string s;
    while (n > 0) {
        n--;
        s += char('A' + n % 26);
        n /= 26;
    }
    reverse(s.begin(), s.end());
    return s;
}

int main() {
    cout << "\"" << columnTitle(1) << "\"" << endl;  // "A"
    cout << "\"" << columnTitle(27) << "\"" << endl; // "AA"
    return 0;
}
