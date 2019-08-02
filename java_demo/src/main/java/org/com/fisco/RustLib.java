package org.com.fisco;

public class RustLib {

    public static native int add(int v1, int v2);
    public static native Person getPerson(String name, int age);
}
