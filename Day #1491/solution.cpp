// Demonstrates ad-hoc (function overloading), parametric (templates),
// and subtype (virtual dispatch) polymorphism. All demos run in main.
#include <bits/stdc++.h>
using namespace std;

// Ad-hoc polymorphism: same name, different signatures.
int add(int a, int b) { return a + b; }
string add(const string& a, const string& b) { return a + b; }

// Parametric polymorphism: one definition works for any type T.
template <typename T>
T identity(T x) { return x; }

// Subtype polymorphism: base reference dispatches to derived override.
struct Animal { virtual string speak() const { return "..."; } virtual ~Animal() {} };
struct Dog : Animal { string speak() const override { return "Woof"; } };
struct Cat : Animal { string speak() const override { return "Meow"; } };

int main() {
    cout << "Ad-hoc (overloading): same name, chosen by argument types.\n";
    cout << "  add(2, 3) = " << add(2, 3) << "\n";
    cout << "  add(\"foo\", \"bar\") = " << add(string("foo"), string("bar")) << "\n";

    cout << "Parametric (generics): one definition, any type.\n";
    cout << "  identity(42) = " << identity(42) << "\n";
    cout << "  identity(\"hi\") = " << identity(string("hi")) << "\n";

    cout << "Subtype (dynamic dispatch): base ref calls overridden method.\n";
    vector<Animal*> zoo = { new Dog(), new Cat() };
    for (Animal* a : zoo) cout << "  " << a->speak() << "\n";
    for (Animal* a : zoo) delete a;
    return 0;
}
