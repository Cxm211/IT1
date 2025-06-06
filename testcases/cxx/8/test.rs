

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        type Canvas;
        type Surface;
    
        fn canvas(self: Pin<&mut Surface>) -> Pin<&mut Canvas>;
    }
}