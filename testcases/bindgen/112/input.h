template <class> class RefPtr {};
class b;
class c;
class B {
  c *d;
};
class e {
  typedef b a;
};
class f {
public:
  f(B);
};
template <typename T> class i {
  typedef T g;
  RefPtr<g> h;
};
class l {
  i<int> j;
};
class c {
  l k;
};
class b : f {};
e Servo_Element_GetSnapshot();