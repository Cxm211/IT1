// bindgen-flags: --opaque-type "B" --whitelist-type "C"

struct A;
class B {
  static A a;
};
class C {
  B b;
};