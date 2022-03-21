/// The policy for how to handle rendering a small frame on a big texture

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum TextureSizeMismatchRenderPolicy {
    TopLeft,
    Center,
}

