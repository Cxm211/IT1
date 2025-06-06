struct B{
    B();
    ~B();
};

// Bindgen incorrectly demangles this as ??_DB@@QEAAXXZ 
// real: ??1B@@QEAA@XZ
B::~B() {}

class C{
public:
    C();
    ~C();
};

// Bindgen incorrectly demangles this as ??_DC@@QEAAXXZ (same as struct)
// symbol: ??1C@@AEAA@XZ (note the A in AEAA)
C::~C() {}