class Bar;

class Foo
{
public:
  void bar(Bar&);
  typedef decltype(&Foo::bar) Getter;
};