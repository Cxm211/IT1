enum class StyleTimingKeyword : unsigned char {
  Foo,
  Bar,
};

enum class StyleStepPosition : unsigned char {
  Baz,
  Bar,
};

template<typename Integer, typename Number>
union StyleTimingFunction {
  enum class Tag : unsigned char {
    Keyword,
    CubicBezier,
    Steps
  };

  struct Keyword_Body {
    Tag tag;
    StyleTimingKeyword _0;
  };

  struct CubicBezier_Body {
    Tag tag;
    Number x1;
    Number y1;
    Number x2;
    Number y2;
  };

  struct Steps_Body {
    Tag tag;
    Integer _0;
    StyleStepPosition _1;
  };

  struct {
    Tag tag;
  };
  Keyword_Body keyword;
  CubicBezier_Body cubic_bezier;
  Steps_Body steps;
};

struct Foo {
  StyleTimingFunction<int, float> f;
};