// B is a rotation of A iff same length and B is a substring of A+A.
// Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

bool isRotation(const string& a, const string& b) {
    return a.size() == b.size() && (a + a).find(b) != string::npos;
}

int main() {
    cout << (isRotation("abcde", "cdeab") ? "true" : "false") << "\n";
    cout << (isRotation("abc", "acb") ? "true" : "false") << "\n";
    return 0;
}
