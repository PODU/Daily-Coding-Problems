// Day 805: Spreadsheet column number -> alphabetical label (bijective base 26).
// Repeatedly take (n-1)%26 for the letter, then n=(n-1)/26. Time O(log n), Space O(log n).
#include <bits/stdc++.h>
using namespace std;

string columnLabel(long long n) {
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
    cout << "\"" << columnLabel(1) << "\"\n";  // "A"
    cout << "\"" << columnLabel(27) << "\"\n"; // "AA"
    return 0;
}
