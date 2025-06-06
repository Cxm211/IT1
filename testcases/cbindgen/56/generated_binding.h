#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Matrix {
  float data[20];
};

struct Data {
  enum class Tag {
    Something,
    Matrix,
  };

  struct Matrix_Body {
    Matrix _0;

    bool operator==(const Matrix_Body& other) const {
      return _0 == other._0;
    }
  };

  Tag tag;
  union {
    Matrix_Body matrix;
  };

  bool operator==(const Data& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Matrix: return matrix == other.matrix;
      default: break;
    }
    return true;
  }
};

extern "C" {

Data *new_data();

}  // extern "C"
