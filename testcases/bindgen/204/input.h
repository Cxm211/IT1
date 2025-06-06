template <typename T>
struct Transform {
  T t;
};

template <typename A, typename B, typename C>
struct TransformOp {
  A a;
  B b;
  C c;
  struct InterpolateMatrix_Body {
    Transform<TransformOp<A, B, C>> from;
    Transform<TransformOp<A, B, C>> to;
    A a;
  };
};