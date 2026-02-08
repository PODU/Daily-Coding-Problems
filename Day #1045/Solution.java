// Demonstrates three kinds of polymorphism: ad-hoc (overloading), parametric
// (generics), subtype (override via base reference). O(1) demo.
import java.util.List;
import java.util.Arrays;

public class Solution {
    // Ad-hoc: method overloading
    static int add(int a, int b) { return a + b; }
    static String add(String a, String b) { return a + b; }

    // Parametric: generic method works for any type
    static <T> T first(List<T> v) { return v.get(0); }

    // Subtype: base type with method overridden by subclasses
    interface Animal { String speak(); }
    static class Dog implements Animal { public String speak() { return "Woof"; } }
    static class Cat implements Animal { public String speak() { return "Meow"; } }

    public static void main(String[] args) {
        System.out.println("Ad-hoc polymorphism: same name, different argument types (overloading).");
        System.out.println("Ad-hoc: add(2,3)=" + add(2, 3) + ", add(\"a\",\"b\")=" + add("a", "b"));

        System.out.println("Parametric polymorphism: one generic definition works for any type.");
        System.out.println("Parametric: first([1,2,3])=" + first(Arrays.asList(1, 2, 3))
                + ", first([\"x\",\"y\"])=" + first(Arrays.asList("x", "y")));

        System.out.println("Subtype polymorphism: a base reference dispatches to the subclass override.");
        Animal dog = new Dog();
        Animal cat = new Cat();
        System.out.println("Subtype: Dog says " + dog.speak() + ", Cat says " + cat.speak());
    }
}
