use crate::{
    bone::Bone,
    c::{c_float, spAttachment, spRegionAttachment, spRegionAttachment_computeWorldVertices},
    c_interface::SyncPtr,
};

#[cfg(feature = "mint")]
use mint::Vector2;

/// An attachment which draws a texture.
///
/// [Spine API Reference](http://esotericsoftware.com/spine-api-reference#RegionAttachment)
#[derive(Debug)]
pub struct RegionAttachment {
    c_region_attachment: SyncPtr<spRegionAttachment>,
}

impl RegionAttachment {
    pub fn new_from_ptr(c_region_attachment: *const spRegionAttachment) -> Self {
        Self {
            c_region_attachment: SyncPtr(c_region_attachment as *mut spRegionAttachment),
        }
    }

    fn attachment(&self) -> &spAttachment {
        unsafe { &self.c_ptr_ref().super_0 }
    }

    pub unsafe fn compute_world_vertices(
        &self,
        bone: &Bone,
        vertices: &mut [f32],
        offset: i32,
        stride: i32,
    ) {
        spRegionAttachment_computeWorldVertices(
            self.c_ptr() as *const spRegionAttachment as *mut spRegionAttachment,
            bone.c_ptr(),
            vertices.as_mut_ptr(),
            offset,
            stride,
        );
    }

    c_attachment_accessors!();
    c_accessor_string!(path, path);
    c_accessor!(x, x, f32);
    c_accessor!(y, y, f32);
    c_accessor!(scale_x, scaleX, f32);
    c_accessor!(scale_y, scaleY, f32);
    c_accessor!(rotation, rotation, f32);
    c_accessor!(width, width, f32);
    c_accessor!(height, height, f32);
    c_accessor_color!(color, color);
    c_accessor_passthrough!(uvs, uvs, [c_float; 8]);
    c_accessor_passthrough!(offset, offset, [c_float; 8]);
    c_accessor_renderer_object!();
    c_ptr!(c_region_attachment, spRegionAttachment);

    // TODO: sequence accessor
}

#[cfg(feature = "mint")]
impl RegionAttachment {
    pub fn position(&self) -> Vector2<f32> {
        Vector2 {
            x: self.x(),
            y: self.y(),
        }
    }

    pub fn scale(&self) -> Vector2<f32> {
        Vector2 {
            x: self.scale_x(),
            y: self.scale_y(),
        }
    }

    pub fn size(&self) -> Vector2<f32> {
        Vector2 {
            x: self.width(),
            y: self.height(),
        }
    }
}
