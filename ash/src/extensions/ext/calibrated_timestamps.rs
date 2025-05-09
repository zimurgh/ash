//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_calibrated_timestamps.html>

use crate::read_into_uninitialized_vector;
use crate::vk;
use crate::VkResult;
use alloc::vec::Vec;
use core::mem;

impl crate::ext::calibrated_timestamps::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetCalibratedTimestampsEXT.html>
    ///
    /// Returns a tuple containing `(timestamps, max_deviation)`
    #[inline]
    pub unsafe fn get_calibrated_timestamps(
        &self,
        info: &[vk::CalibratedTimestampInfoEXT<'_>],
    ) -> VkResult<(Vec<u64>, u64)> {
        let mut timestamps = Vec::with_capacity(info.len());
        let mut max_deviation = mem::MaybeUninit::uninit();
        let max_deviation = (self.fp.get_calibrated_timestamps_ext)(
            self.handle,
            info.len() as u32,
            info.as_ptr(),
            timestamps.as_mut_ptr(),
            max_deviation.as_mut_ptr(),
        )
        .assume_init_on_success(max_deviation)?;
        timestamps.set_len(info.len());
        Ok((timestamps, max_deviation))
    }
}

impl crate::ext::calibrated_timestamps::Instance {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceCalibrateableTimeDomainsEXT.html>
    #[inline]
    pub unsafe fn get_physical_device_calibrateable_time_domains(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> VkResult<Vec<vk::TimeDomainEXT>> {
        read_into_uninitialized_vector(|count, data| {
            (self.fp.get_physical_device_calibrateable_time_domains_ext)(
                physical_device,
                count,
                data,
            )
        })
    }
}
