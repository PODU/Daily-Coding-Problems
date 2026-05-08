// Demonstrates ad-hoc (method overloading), parametric (generics),
// and subtype (interface/override dispatch) polymorphism. Runs in main.
import java.util.List;

public class Solution {
    // Ad-hoc polymorphism: same name, different signatures.
    static int add(int a, int b) { return a + b; }
    static String add(String a, String b) { return a + b; }

    // Parametric polymorphism: one definition works for any type T.
    static <T> T identity(T x) { return x; }

    // Subtype polymorphism: base reference dispatches to override.
    interface Animal { String speak(); }
    static class Dog implements Animal { public String speak() { return "Woof"; } }
    static class Cat implements Animal { public String speak() { return "Meow"; } }

    public static void main(String[] args) {
        System.out.println("Ad-hoc (overloading): same name, chosen by argument types.");
        System.out.println("  add(2, 3) = " + add(2, 3));
        System.out.println("  add(\"foo\", \"bar\") = " + add("foo", "bar"));

        System.out.println("Parametric (generics): one definition, any type.");
        System.out.println("  identity(42) = " + identity(42));
        System.out.println("  identity(\"hi\") = " + identity("hi"));

        System.out.println("Subtype (dynamic dispatch): base ref calls overridden method.");
        for (Animal a : List.of(new Dog(), new Cat())) {
            System.out.println("  " + a.speak());
        }
    }
}
