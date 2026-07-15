// String rotation check: B is a rotation of A iff |A|==|B| and B is a substring of A+A.
// Time: O(n) with substring search (here std::string::find). Space: O(n).
#include <bits/stdc++.h>
using namespace std;

bool isRotation(const string& a, const string& b) {
    return a.size() == b.size() && (a + a).find(b) != string::npos;
}

int main() {
    cout << boolalpha;
    cout << isRotation("abcde", "cdeab") << "\n"; // true
    cout << isRotation("abc", "acb") << "\n";     // false
    return 0;
}
