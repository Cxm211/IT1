template <int b> struct c { char d[b]; };
template <int e> struct f { static const long g = e; };
template <int e> struct h { static const long g = e; };
class i {
  int j;
};
template <typename k, class l> class m : l {
  struct n {
    static const long g = sizeof(k);
  };
  static const long a = f<n::g>::g;
  static const long b = h<a * n::g>::g;
  k o;
  long p;
  long q;
  long r;
  c<b> s;
  bool t;
};
template <typename k> class G {
  using af = void *;
  af ag;
  k ah;
};
template <typename k> using aj = G<k>;
template <typename k> class u {
  u *a;
  u *am;
  aj<k> an;
};
struct I {
  long ap;
};
namespace JS {
class v {
  m<I, i> ar;
};
class AutoIdVector : u<v> {};
}