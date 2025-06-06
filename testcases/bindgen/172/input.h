template <typename> struct a { typedef int b; };
template <typename = a<int>> class c;
template <typename e> class c {
  typedef typename e::b b;
  c &d(long, b);
};
template <> c<> &c<>::d(long, b);