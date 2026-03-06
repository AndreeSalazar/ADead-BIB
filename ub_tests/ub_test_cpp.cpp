class Object {
public:
    int value;
    Object() : value(0) {}
};

int main() {
    Object* obj = nullptr;
    int a = obj->value; // Null pointer dereference (L9)
    
    int arr[5];
    arr[10] = 42; // Out of bounds (L12)
    
    int x = 0;
    int y = 10 / x; // Div zero (L15)
    
    return a;
}
