template <class temp>
class MyClass {
public:
	MyClass(){}
	temp options;
	static MyClass *first;
};
template <class temp> MyClass<temp> *MyClass<temp>::first = nullptr;