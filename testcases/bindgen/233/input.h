class Base {};
        
template <class BaseT>
class CallArgsBase : BaseT {
    int *c;
    unsigned d;
};

class CallArgs : CallArgsBase<Base> {};