// ADead-BIB HIP-CPU fallback stub

#[derive(Debug, Clone, Copy, Default)]
pub struct Dim3 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

#[derive(Debug, Clone, Default)]
pub struct HipCpuConfig;

#[derive(Debug, Default)]
pub struct HipCpuRuntime;

#[derive(Debug, Clone, Default)]
pub struct HipCpuStats;

#[derive(Debug, Clone, Copy)]
pub struct SendPtr<T>(pub *mut T);

unsafe impl<T> Send for SendPtr<T> {}
unsafe impl<T> Sync for SendPtr<T> {}

#[derive(Debug, Clone, Copy, Default)]
pub struct ThreadIdx {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}
