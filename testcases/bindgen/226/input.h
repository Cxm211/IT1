template <typename, typename = int> class A {
  class B;
};

template <typename _CharT, typename _Traits> class A<_CharT, _Traits>::B {
  _Traits traits_type;
};

class C {
protected:
  void m_fn1(A<char>);
};

class Test {
  C fChain;
};