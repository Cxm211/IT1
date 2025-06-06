struct simple {
 int a, b, c;
};

template<class ty>
struct templated {
 ty a, b, c;
};

typedef templated<int> int_tmpl;