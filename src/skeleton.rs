use std::{ffi::CString, sync::Arc};

use crate::{
    bone::Bone,
    c::{
        spBone, spSkeleton, spSkeletonData, spSkeleton_create, spSkeleton_dispose,
        spSkeleton_setBonesToSetupPose, spSkeleton_setSkin, spSkeleton_setSkinByName,
        spSkeleton_setSlotsToSetupPose, spSkeleton_setToSetupPose, spSkeleton_updateCache,
        spSkeleton_updateWorldTransform, spSkin, spSlot,
    },
    c_interface::{CTmpMut, CTmpRef, NewFromPtr, SyncPtr},
    error::Error,
    skeleton_data::SkeletonData,
    skin::Skin,
    slot::Slot,
};

/// A live Skeleton instance created from [SkeletonData](struct.SkeletonData.html).
#[derive(Debug)]
pub struct Skeleton {
    c_skeleton: SyncPtr<spSkeleton>,
    owns_memory: bool,
    _skeleton_data: Arc<SkeletonData>,
}

impl Skeleton {
    /// Create a new instance of the skeleton loaded in [SkeletonData](struct.SkeletonData.html).
    ///
    /// See [SkeletonJson](struct.SkeletonJson.html) or
    /// [SkeletonBinary](struct.SkeletonBinary.html) for a complete example of loading a skeleton.
    pub fn new(skeleton_data: Arc<SkeletonData>) -> Self {
        let c_skeleton = unsafe { spSkeleton_create(skeleton_data.c_ptr()) };
        Self {
            c_skeleton: SyncPtr(c_skeleton),
            owns_memory: true,
            _skeleton_data: skeleton_data,
        }
    }

    pub fn update_cache(&mut self) {
        unsafe {
            spSkeleton_updateCache(self.c_ptr());
        }
    }

    pub fn update_world_transform(&mut self) {
        unsafe {
            spSkeleton_updateWorldTransform(self.c_ptr());
        }
    }

    pub fn set_to_setup_pose(&mut self) {
        unsafe {
            spSkeleton_setToSetupPose(self.c_ptr());
        }
    }

    pub fn set_bones_to_setup_pose(&mut self) {
        unsafe {
            spSkeleton_setBonesToSetupPose(self.c_ptr());
        }
    }

    pub fn set_slots_to_setup_pose(&mut self) {
        unsafe {
            spSkeleton_setSlotsToSetupPose(self.c_ptr());
        }
    }

    pub fn set_skin(&mut self, skin: &Skin) {
        unsafe { spSkeleton_setSkin(self.c_ptr(), skin.c_ptr()) };
    }

    pub unsafe fn set_skin_by_name_unchecked(&mut self, skin_name: &str) {
        let c_skin_name = CString::new(skin_name).unwrap();
        spSkeleton_setSkinByName(self.c_ptr(), c_skin_name.as_ptr());
    }

    pub fn set_skin_by_name(&mut self, skin_name: &str) -> Result<(), Error> {
        if self
            .data()
            .skins()
            .find(|skin| skin.name() == skin_name)
            .is_some()
        {
            unsafe { self.set_skin_by_name_unchecked(skin_name) };
            Ok(())
        } else {
            Err(Error::NotFound)
        }
    }

    pub fn bone_root(&self) -> CTmpRef<Skeleton, Bone> {
        CTmpRef::new(self, unsafe { Bone::new_from_ptr(self.c_ptr_mut().root) })
    }

    pub fn bone_root_mut(&mut self) -> CTmpMut<Skeleton, Bone> {
        CTmpMut::new(self, unsafe { Bone::new_from_ptr(self.c_ptr_mut().root) })
    }

    pub fn find_bone(&self, name: &str) -> Option<CTmpRef<Skeleton, Bone>> {
        self.bones().find(|bone| bone.data().name() == name)
    }

    pub fn find_bone_mut(&mut self, name: &str) -> Option<CTmpMut<Skeleton, Bone>> {
        self.bones_mut().find(|bone| bone.data().name() == name)
    }

    pub fn find_slot(&self, name: &str) -> Option<CTmpRef<Skeleton, Slot>> {
        self.slots().find(|slot| slot.data().name() == name)
    }

    pub fn find_slot_mut(&mut self, name: &str) -> Option<CTmpMut<Skeleton, Slot>> {
        self.slots_mut().find(|slot| slot.data().name() == name)
    }

    // TODO: iterators for ik, transform, path constraints

    c_accessor_tmp_ptr!(data, data_mut, data, SkeletonData, spSkeletonData);
    c_accessor_color_mut!(color, color_mut, color);
    c_accessor!(bones_count, bonesCount, i32);
    c_accessor!(slots_count, slotsCount, i32);
    c_accessor!(ik_contraints_count, ikConstraintsCount, i32);
    c_accessor!(transform_contraints_count, transformConstraintsCount, i32);
    c_accessor!(path_contraints_count, pathConstraintsCount, i32);
    c_accessor_mut!(scale_x, set_scale_x, scaleX, f32);
    c_accessor_mut!(scale_y, set_scale_y, scaleY, f32);
    c_accessor_mut!(x, set_x, x, f32);
    c_accessor_mut!(y, set_y, y, f32);
    c_accessor_array!(
        bones,
        bones_mut,
        bone_at_index,
        bone_at_index_mut,
        Skeleton,
        Bone,
        spBone,
        bones,
        bones_count
    );
    c_accessor_array!(
        slots,
        slots_mut,
        slot_at_index,
        slot_at_index_mut,
        Skeleton,
        Slot,
        spSlot,
        slots,
        slots_count
    );
    c_accessor_array!(
        draw_order,
        draw_order_mut,
        draw_order_at_index,
        draw_order_at_index_mut,
        Skeleton,
        Slot,
        spSlot,
        drawOrder,
        slots_count
    );
    c_accessor_tmp_ptr_optional!(skin, skin_mut, skin, Skin, spSkin);
    c_ptr!(c_skeleton, spSkeleton);
}

impl Drop for Skeleton {
    fn drop(&mut self) {
        if self.owns_memory {
            unsafe {
                spSkeleton_dispose(self.c_skeleton.0);
            }
        }
    }
}

/*
/* Sets the skin used to look up attachments before looking in the SkeletonData defaultSkin. Attachments from the new skin are
 * attached if the corresponding attachment from the old skin was attached. If there was no old skin, each slot's setup mode
 * attachment is attached from the new skin.
 * @param skin May be 0.*/
SP_API void spSkeleton_setSkin(spSkeleton *self, spSkin *skin);
/* Returns 0 if the skin was not found. See spSkeleton_setSkin.
 * @param skinName May be 0. */
SP_API int spSkeleton_setSkinByName(spSkeleton *self, const char *skinName);

/* Returns 0 if the slot or attachment was not found. */
SP_API spAttachment *
spSkeleton_getAttachmentForSlotName(const spSkeleton *self, const char *slotName, const char *attachmentName);
/* Returns 0 if the slot or attachment was not found. */
SP_API spAttachment *
spSkeleton_getAttachmentForSlotIndex(const spSkeleton *self, int slotIndex, const char *attachmentName);
/* Returns 0 if the slot or attachment was not found.
 * @param attachmentName May be 0. */
SP_API int spSkeleton_setAttachment(spSkeleton *self, const char *slotName, const char *attachmentName);

/* Returns 0 if the IK constraint was not found. */
SP_API spIkConstraint *spSkeleton_findIkConstraint(const spSkeleton *self, const char *constraintName);

/* Returns 0 if the transform constraint was not found. */
SP_API spTransformConstraint *spSkeleton_findTransformConstraint(const spSkeleton *self, const char *constraintName);

/* Returns 0 if the path constraint was not found. */
SP_API spPathConstraint *spSkeleton_findPathConstraint(const spSkeleton *self, const char *constraintName);*/
