// Gray code generation via reflect formula i ^ (i>>1). Time O(2^n), Space O(2^n).
#include <iostream>
#include <vector>
#include <string>
using namespace std;

vector<string> grayCode(int n) {
    vector<string> res;
    for (int i = 0; i < (1 << n); i++) {
        int g = i ^ (i >> 1);
        string s(n, '0');
        for (int b = 0; b < n; b++)
            if (g & (1 << (n - 1 - b))) s[b] = '1';
        res.push_back(s);
    }
    return res;
}

int main() {
    int n = 2;
    vector<string> codes = grayCode(n);
    cout << "[";
    for (size_t i = 0; i < codes.size(); i++) {
        cout << codes[i];
        if (i + 1 < codes.size()) cout << ", ";
    }
    cout << "]" << endl;
    return 0;
}
