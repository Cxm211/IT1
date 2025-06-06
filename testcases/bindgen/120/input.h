struct Foo {};
typedef union jvalue {
    unsigned char z;
    struct Foo *p;
} jvalue;