typedef __declspec(align(16)) union TestUnion
{
    unsigned long Longs[2];
    unsigned int  Ints[4];
} TestUnion;