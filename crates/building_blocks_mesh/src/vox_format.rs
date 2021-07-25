use vox_format::types::Color;

use crate::IsOpaque;

impl IsOpaque for Color {
    fn is_opaque(&self) -> bool {
        self.a != 255
    }
}
