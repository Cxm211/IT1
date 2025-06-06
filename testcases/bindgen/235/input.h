

class A {
public:
  int foo;
};

class B: public virtual A {
public:
  int bar;
};

class C: public virtual A {
public:
  int baz;
};

class D: public C, public B {
public:
  int bazz;
};
