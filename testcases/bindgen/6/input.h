namespace {
namespace {
namespace {
template <typename a> class b {
public:
  operator a *() { c; }
  static __thread a *c;
};
template <typename a> __thread a *b<a>::c;
template <typename a> class d : public b<a> {};
template <typename e, typename f> class g {
public:
  class context {
  public:
    context();
    e c;
  };
  static f *contains() {
    context *elem = h;
    elem->c;
  }
  static d<context> h;
};
class i;
class j {
public:
  typedef g<j, i> k;
};
class l : j {
  ~l();
};
l::~l() { k::contains; }
} // namespace
} // namespace
} // namespace