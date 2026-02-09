use std::fmt;
use wgpu::{Device, Sampler};

/// Errors that can occur during sampler operations
#[derive(Debug)]
pub enum SamplerError {
    /// Invalid sampler configuration
    InvalidConfiguration(String),
}

impl fmt::Display for SamplerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SamplerError::InvalidConfiguration(msg) => {
                write!(f, "Invalid sampler configuration: {}", msg)
            }
        }
    }
}

impl std::error::Error for SamplerError {}

/// Address mode for texture sampling outside [0, 1] range
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressMode {
    /// Clamp to edge - coordinates outside [0, 1] are clamped to the edge
    ClampToEdge,
    /// Repeat - coordinates wrap around, creating a repeating pattern
    Repeat,
    /// Mirror repeat - coordinates bounce back and forth, creating a mirrored pattern
    MirrorRepeat,
    /// Clamp to border - coordinates outside [0, 1] use border color
    ClampToBorder,
}

impl AddressMode {
    /// Convert to wgpu::AddressMode
    pub fn to_wgpu(&self) -> wgpu::AddressMode {
        match self {
            AddressMode::ClampToEdge => wgpu::AddressMode::ClampToEdge,
            AddressMode::Repeat => wgpu::AddressMode::Repeat,
            AddressMode::MirrorRepeat => wgpu::AddressMode::MirrorRepeat,
            AddressMode::ClampToBorder => wgpu::AddressMode::ClampToBorder,
        }
    }
}

/// Filter mode for texture sampling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterMode {
    /// Nearest neighbor filtering - use the value of the nearest texel
    Nearest,
    /// Linear filtering - interpolate between nearby texels
    Linear,
}

impl FilterMode {
    /// Convert to wgpu::FilterMode
    pub fn to_wgpu(&self) -> wgpu::FilterMode {
        match self {
            FilterMode::Nearest => wgpu::FilterMode::Nearest,
            FilterMode::Linear => wgpu::FilterMode::Linear,
        }
    }
}

/// Mipmap filter mode for level of detail (LOD) sampling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MipmapFilterMode {
    /// Nearest neighbor filtering between mipmap levels
    Nearest,
    /// Linear filtering between mipmap levels
    Linear,
}

impl MipmapFilterMode {
    /// Convert to wgpu::MipmapFilterMode
    pub fn to_wgpu(&self) -> wgpu::MipmapFilterMode {
        match self {
            MipmapFilterMode::Nearest => wgpu::MipmapFilterMode::Nearest,
            MipmapFilterMode::Linear => wgpu::MipmapFilterMode::Linear,
        }
    }
}

/// Comparison function for depth/stencil testing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareFunction {
    /// Never pass
    Never,
    /// Pass if new value is less than existing value
    Less,
    /// Pass if new value equals existing value
    Equal,
    /// Pass if new value is less than or equal to existing value
    LessEqual,
    /// Pass if new value is greater than existing value
    Greater,
    /// Pass if new value does not equal existing value
    NotEqual,
    /// Pass if new value is greater than or equal to existing value
    GreaterEqual,
    /// Always pass
    Always,
}

impl CompareFunction {
    /// Convert to wgpu::CompareFunction
    pub fn to_wgpu(&self) -> wgpu::CompareFunction {
        match self {
            CompareFunction::Never => wgpu::CompareFunction::Never,
            CompareFunction::Less => wgpu::CompareFunction::Less,
            CompareFunction::Equal => wgpu::CompareFunction::Equal,
            CompareFunction::LessEqual => wgpu::CompareFunction::LessEqual,
            CompareFunction::Greater => wgpu::CompareFunction::Greater,
            CompareFunction::NotEqual => wgpu::CompareFunction::NotEqual,
            CompareFunction::GreaterEqual => wgpu::CompareFunction::GreaterEqual,
            CompareFunction::Always => wgpu::CompareFunction::Always,
        }
    }
}

/// Descriptor for creating a GPU sampler
///
/// Samplers control how textures are sampled and filtered when accessed in shaders.
#[derive(Debug, Clone)]
pub struct SamplerDescriptor {
    /// Optional label for debugging
    label: Option<String>,
    /// Address mode for the U (horizontal) coordinate
    address_mode_u: AddressMode,
    /// Address mode for the V (vertical) coordinate
    address_mode_v: AddressMode,
    /// Address mode for the W (depth) coordinate
    address_mode_w: AddressMode,
    /// Magnification filter mode (when pixel is smaller than texel, zooming in)
    mag_filter: FilterMode,
    /// Minification filter mode (when pixel is larger than texel, zooming out)
    min_filter: FilterMode,
    /// Mipmap filter mode for level of detail
    mipmap_filter: MipmapFilterMode,
    /// Minimum LOD clamp value
    lod_min_clamp: f32,
    /// Maximum LOD clamp value
    lod_max_clamp: f32,
    /// Optional comparison function for depth/stencil testing
    compare: Option<CompareFunction>,
    /// Anisotropic filtering level (1-16). Values greater than 1 enable anisotropic filtering.
    anisotropy_clamp: u16,
    /// Border color (only used when address mode is ClampToBorder)
    border_color: Option<wgpu::SamplerBorderColor>,
}

impl SamplerDescriptor {
    /// Create a new sampler descriptor with default values
    ///
    /// # Arguments
    /// * `label` - Optional label for debugging
    ///
    /// # Examples
    /// ```
    /// use wgpu_playground_core::sampler::SamplerDescriptor;
    ///
    /// let descriptor = SamplerDescriptor::new(Some("my_sampler"));
    /// ```
    pub fn new(label: Option<&str>) -> Self {
        Self {
            label: label.map(String::from),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            mipmap_filter: MipmapFilterMode::Nearest,
            lod_min_clamp: 0.0,
            lod_max_clamp: 32.0,
            compare: None,
            anisotropy_clamp: 1,
            border_color: None,
        }
    }

    /// Set the address mode for all coordinates (U, V, W)
    ///
    /// # Arguments
    /// * `mode` - The address mode to use for all coordinates
    ///
    /// # Examples
    /// ```
    /// use wgpu_playground_core::sampler::{SamplerDescriptor, AddressMode};
    ///
    /// let descriptor = SamplerDescriptor::new(None)
    ///     .with_address_mode(AddressMode::Repeat);
    /// ```
    pub fn with_address_mode(mut self, mode: AddressMode) -> Self {
        self.address_mode_u = mode;
        self.address_mode_v = mode;
        self.address_mode_w = mode;
        self
    }

    /// Set the address mode for U coordinate
    pub fn with_address_mode_u(mut self, mode: AddressMode) -> Self {
        self.address_mode_u = mode;
        self
    }

    /// Set the address mode for V coordinate
    pub fn with_address_mode_v(mut self, mode: AddressMode) -> Self {
        self.address_mode_v = mode;
        self
    }

    /// Set the address mode for W coordinate
    pub fn with_address_mode_w(mut self, mode: AddressMode) -> Self {
        self.address_mode_w = mode;
        self
    }

    /// Set the magnification filter mode
    ///
    /// # Arguments
    /// * `filter` - The filter mode to use when magnifying
    pub fn with_mag_filter(mut self, filter: FilterMode) -> Self {
        self.mag_filter = filter;
        self
    }

    /// Set the minification filter mode
    ///
    /// # Arguments
    /// * `filter` - The filter mode to use when minifying
    pub fn with_min_filter(mut self, filter: FilterMode) -> Self {
        self.min_filter = filter;
        self
    }

    /// Set both magnification and minification filter modes
    ///
    /// # Arguments
    /// * `filter` - The filter mode to use for both mag and min
    ///
    /// # Examples
    /// ```
    /// use wgpu_playground_core::sampler::{SamplerDescriptor, FilterMode};
    ///
    /// let descriptor = SamplerDescriptor::new(None)
    ///     .with_filter(FilterMode::Linear);
    /// ```
    pub fn with_filter(mut self, filter: FilterMode) -> Self {
        self.mag_filter = filter;
        self.min_filter = filter;
        self
    }

    /// Set the mipmap filter mode
    ///
    /// # Arguments
    /// * `filter` - The filter mode to use for mipmap levels
    pub fn with_mipmap_filter(mut self, filter: MipmapFilterMode) -> Self {
        self.mipmap_filter = filter;
        self
    }

    /// Set the minimum LOD clamp value
    ///
    /// # Arguments
    /// * `lod` - The minimum LOD value
    pub fn with_lod_min_clamp(mut self, lod: f32) -> Self {
        self.lod_min_clamp = lod;
        self
    }

    /// Set the maximum LOD clamp value
    ///
    /// # Arguments
    /// * `lod` - The maximum LOD value
    pub fn with_lod_max_clamp(mut self, lod: f32) -> Self {
        self.lod_max_clamp = lod;
        self
    }

    /// Set the LOD clamp range
    ///
    /// # Arguments
    /// * `min` - The minimum LOD value
    /// * `max` - The maximum LOD value
    ///
    /// # Examples
    /// ```
    /// use wgpu_playground_core::sampler::SamplerDescriptor;
    ///
    /// let descriptor = SamplerDescriptor::new(None)
    ///     .with_lod_clamp(0.0, 10.0);
    /// ```
    pub fn with_lod_clamp(mut self, min: f32, max: f32) -> Self {
        self.lod_min_clamp = min;
        self.lod_max_clamp = max;
        self
    }

    /// Set the comparison function for depth/stencil testing
    ///
    /// # Arguments
    /// * `compare` - The comparison function to use
    ///
    /// # Examples
    /// ```
    /// use wgpu_playground_core::sampler::{SamplerDescriptor, CompareFunction};
    ///
    /// let descriptor = SamplerDescriptor::new(None)
    ///     .with_compare(CompareFunction::Less);
    /// ```
    pub fn with_compare(mut self, compare: CompareFunction) -> Self {
        self.compare = Some(compare);
        self
    }

    /// Set the anisotropic filtering level
    ///
    /// # Arguments
    /// * `level` - Anisotropy level (1-16). 1 disables anisotropic filtering.
    ///
    /// # Examples
    /// ```
    /// use wgpu_playground_core::sampler::SamplerDescriptor;
    ///
    /// let descriptor = SamplerDescriptor::new(None)
    ///     .with_anisotropy(16);
    /// ```
    pub fn with_anisotropy(mut self, level: u16) -> Self {
        self.anisotropy_clamp = level;
        self
    }

    /// Set the border color (only used when address mode is ClampToBorder)
    ///
    /// # Arguments
    /// * `color` - The border color to use
    pub fn with_border_color(mut self, color: wgpu::SamplerBorderColor) -> Self {
        self.border_color = Some(color);
        self
    }

    /// Get the label
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Get the address mode for U coordinate
    pub fn address_mode_u(&self) -> AddressMode {
        self.address_mode_u
    }

    /// Get the address mode for V coordinate
    pub fn address_mode_v(&self) -> AddressMode {
        self.address_mode_v
    }

    /// Get the address mode for W coordinate
    pub fn address_mode_w(&self) -> AddressMode {
        self.address_mode_w
    }

    /// Get the magnification filter mode
    pub fn mag_filter(&self) -> FilterMode {
        self.mag_filter
    }

    /// Get the minification filter mode
    pub fn min_filter(&self) -> FilterMode {
        self.min_filter
    }

    /// Get the mipmap filter mode
    pub fn mipmap_filter(&self) -> MipmapFilterMode {
        self.mipmap_filter
    }

    /// Get the minimum LOD clamp value
    pub fn lod_min_clamp(&self) -> f32 {
        self.lod_min_clamp
    }

    /// Get the maximum LOD clamp value
    pub fn lod_max_clamp(&self) -> f32 {
        self.lod_max_clamp
    }

    /// Get the comparison function
    pub fn compare(&self) -> Option<CompareFunction> {
        self.compare
    }

    /// Get the anisotropic filtering level
    pub fn anisotropy_clamp(&self) -> u16 {
        self.anisotropy_clamp
    }

    /// Get the border color
    pub fn border_color(&self) -> Option<wgpu::SamplerBorderColor> {
        self.border_color
    }

    /// Validate the sampler descriptor
    ///
    /// Checks for:
    /// - Valid LOD range (min <= max)
    /// - Valid anisotropy level (1-16)
    /// - Border color is specified when using ClampToBorder address mode
    ///
    /// # Returns
    /// Ok(()) if valid, Err with SamplerError if invalid
    pub fn validate(&self) -> Result<(), SamplerError> {
        if self.lod_min_clamp > self.lod_max_clamp {
            return Err(SamplerError::InvalidConfiguration(
                "lod_min_clamp must be less than or equal to lod_max_clamp".to_string(),
            ));
        }

        if self.anisotropy_clamp < 1 || self.anisotropy_clamp > 16 {
            return Err(SamplerError::InvalidConfiguration(
                "anisotropy_clamp must be between 1 and 16".to_string(),
            ));
        }

        // Check that border color is specified when using ClampToBorder
        if (self.address_mode_u == AddressMode::ClampToBorder
            || self.address_mode_v == AddressMode::ClampToBorder
            || self.address_mode_w == AddressMode::ClampToBorder)
            && self.border_color.is_none()
        {
            return Err(SamplerError::InvalidConfiguration(
                "border_color must be specified when using ClampToBorder address mode".to_string(),
            ));
        }

        Ok(())
    }

    /// Create a wgpu sampler from this descriptor
    ///
    /// This method validates the descriptor and creates the actual sampler.
    ///
    /// # Arguments
    /// * `device` - The wgpu device to create the sampler on
    ///
    /// # Returns
    /// A Result containing the Sampler or a SamplerError
    ///
    /// # Examples
    /// ```no_run
    /// use wgpu_playground_core::sampler::{SamplerDescriptor, AddressMode, FilterMode};
    /// # async fn example(device: &wgpu::Device) {
    /// let descriptor = SamplerDescriptor::new(Some("my_sampler"))
    ///     .with_address_mode(AddressMode::Repeat)
    ///     .with_filter(FilterMode::Linear);
    ///
    /// let sampler = descriptor.create_sampler(device).unwrap();
    /// # }
    /// ```
    pub fn create_sampler(&self, device: &Device) -> Result<Sampler, SamplerError> {
        self.validate()?;

        Ok(device.create_sampler(&wgpu::SamplerDescriptor {
            label: self.label.as_deref(),
            address_mode_u: self.address_mode_u.to_wgpu(),
            address_mode_v: self.address_mode_v.to_wgpu(),
            address_mode_w: self.address_mode_w.to_wgpu(),
            mag_filter: self.mag_filter.to_wgpu(),
            min_filter: self.min_filter.to_wgpu(),
            mipmap_filter: self.mipmap_filter.to_wgpu(),
            lod_min_clamp: self.lod_min_clamp,
            lod_max_clamp: self.lod_max_clamp,
            compare: self.compare.map(|c| c.to_wgpu()),
            anisotropy_clamp: self.anisotropy_clamp,
            border_color: self.border_color,
        }))
    }
}

impl Default for SamplerDescriptor {
    fn default() -> Self {
        Self::new(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_mode_variants() {
        assert_eq!(AddressMode::ClampToEdge, AddressMode::ClampToEdge);
        assert_eq!(AddressMode::Repeat, AddressMode::Repeat);
        assert_eq!(AddressMode::MirrorRepeat, AddressMode::MirrorRepeat);
        assert_eq!(AddressMode::ClampToBorder, AddressMode::ClampToBorder);
    }

    #[test]
    fn test_filter_mode_variants() {
        assert_eq!(FilterMode::Nearest, FilterMode::Nearest);
        assert_eq!(FilterMode::Linear, FilterMode::Linear);
    }

    #[test]
    fn test_mipmap_filter_mode_variants() {
        assert_eq!(MipmapFilterMode::Nearest, MipmapFilterMode::Nearest);
        assert_eq!(MipmapFilterMode::Linear, MipmapFilterMode::Linear);
    }

    #[test]
    fn test_compare_function_variants() {
        assert_eq!(CompareFunction::Never, CompareFunction::Never);
        assert_eq!(CompareFunction::Less, CompareFunction::Less);
        assert_eq!(CompareFunction::Equal, CompareFunction::Equal);
        assert_eq!(CompareFunction::LessEqual, CompareFunction::LessEqual);
        assert_eq!(CompareFunction::Greater, CompareFunction::Greater);
        assert_eq!(CompareFunction::NotEqual, CompareFunction::NotEqual);
        assert_eq!(CompareFunction::GreaterEqual, CompareFunction::GreaterEqual);
        assert_eq!(CompareFunction::Always, CompareFunction::Always);
    }

    #[test]
    fn test_sampler_descriptor_creation() {
        let descriptor = SamplerDescriptor::new(Some("test_sampler"));
        assert_eq!(descriptor.label(), Some("test_sampler"));
        assert_eq!(descriptor.address_mode_u(), AddressMode::ClampToEdge);
        assert_eq!(descriptor.address_mode_v(), AddressMode::ClampToEdge);
        assert_eq!(descriptor.address_mode_w(), AddressMode::ClampToEdge);
        assert_eq!(descriptor.mag_filter(), FilterMode::Nearest);
        assert_eq!(descriptor.min_filter(), FilterMode::Nearest);
        assert_eq!(descriptor.mipmap_filter(), MipmapFilterMode::Nearest);
        assert_eq!(descriptor.lod_min_clamp(), 0.0);
        assert_eq!(descriptor.lod_max_clamp(), 32.0);
        assert_eq!(descriptor.compare(), None);
        assert_eq!(descriptor.anisotropy_clamp(), 1);
        assert_eq!(descriptor.border_color(), None);
    }

    #[test]
    fn test_sampler_with_address_mode() {
        let descriptor = SamplerDescriptor::new(None).with_address_mode(AddressMode::Repeat);

        assert_eq!(descriptor.address_mode_u(), AddressMode::Repeat);
        assert_eq!(descriptor.address_mode_v(), AddressMode::Repeat);
        assert_eq!(descriptor.address_mode_w(), AddressMode::Repeat);
    }

    #[test]
    fn test_sampler_with_individual_address_modes() {
        let descriptor = SamplerDescriptor::new(None)
            .with_address_mode_u(AddressMode::Repeat)
            .with_address_mode_v(AddressMode::MirrorRepeat)
            .with_address_mode_w(AddressMode::ClampToEdge);

        assert_eq!(descriptor.address_mode_u(), AddressMode::Repeat);
        assert_eq!(descriptor.address_mode_v(), AddressMode::MirrorRepeat);
        assert_eq!(descriptor.address_mode_w(), AddressMode::ClampToEdge);
    }

    #[test]
    fn test_sampler_with_filter() {
        let descriptor = SamplerDescriptor::new(None).with_filter(FilterMode::Linear);

        assert_eq!(descriptor.mag_filter(), FilterMode::Linear);
        assert_eq!(descriptor.min_filter(), FilterMode::Linear);
    }

    #[test]
    fn test_sampler_with_individual_filters() {
        let descriptor = SamplerDescriptor::new(None)
            .with_mag_filter(FilterMode::Linear)
            .with_min_filter(FilterMode::Nearest);

        assert_eq!(descriptor.mag_filter(), FilterMode::Linear);
        assert_eq!(descriptor.min_filter(), FilterMode::Nearest);
    }

    #[test]
    fn test_sampler_with_mipmap_filter() {
        let descriptor = SamplerDescriptor::new(None).with_mipmap_filter(MipmapFilterMode::Linear);

        assert_eq!(descriptor.mipmap_filter(), MipmapFilterMode::Linear);
    }

    #[test]
    fn test_sampler_with_lod_clamp() {
        let descriptor = SamplerDescriptor::new(None).with_lod_clamp(1.0, 10.0);

        assert_eq!(descriptor.lod_min_clamp(), 1.0);
        assert_eq!(descriptor.lod_max_clamp(), 10.0);
    }

    #[test]
    fn test_sampler_with_individual_lod_clamps() {
        let descriptor = SamplerDescriptor::new(None)
            .with_lod_min_clamp(2.0)
            .with_lod_max_clamp(8.0);

        assert_eq!(descriptor.lod_min_clamp(), 2.0);
        assert_eq!(descriptor.lod_max_clamp(), 8.0);
    }

    #[test]
    fn test_sampler_with_compare() {
        let descriptor = SamplerDescriptor::new(None).with_compare(CompareFunction::Less);

        assert_eq!(descriptor.compare(), Some(CompareFunction::Less));
    }

    #[test]
    fn test_sampler_with_anisotropy() {
        let descriptor = SamplerDescriptor::new(None).with_anisotropy(16);

        assert_eq!(descriptor.anisotropy_clamp(), 16);
    }

    #[test]
    fn test_sampler_with_border_color() {
        let descriptor =
            SamplerDescriptor::new(None).with_border_color(wgpu::SamplerBorderColor::OpaqueBlack);

        assert_eq!(
            descriptor.border_color(),
            Some(wgpu::SamplerBorderColor::OpaqueBlack)
        );
    }

    #[test]
    fn test_sampler_validation_invalid_lod_range() {
        let descriptor = SamplerDescriptor::new(None).with_lod_clamp(10.0, 5.0);

        let result = descriptor.validate();
        assert!(result.is_err());
        match result {
            Err(SamplerError::InvalidConfiguration(msg)) => {
                assert!(msg.contains("lod_min_clamp"));
            }
            _ => panic!("Expected InvalidConfiguration error"),
        }
    }

    #[test]
    fn test_sampler_validation_invalid_anisotropy_too_low() {
        let descriptor = SamplerDescriptor::new(None).with_anisotropy(0);

        let result = descriptor.validate();
        assert!(result.is_err());
        match result {
            Err(SamplerError::InvalidConfiguration(msg)) => {
                assert!(msg.contains("anisotropy_clamp"));
            }
            _ => panic!("Expected InvalidConfiguration error"),
        }
    }

    #[test]
    fn test_sampler_validation_invalid_anisotropy_too_high() {
        let descriptor = SamplerDescriptor::new(None).with_anisotropy(17);

        let result = descriptor.validate();
        assert!(result.is_err());
        match result {
            Err(SamplerError::InvalidConfiguration(msg)) => {
                assert!(msg.contains("anisotropy_clamp"));
            }
            _ => panic!("Expected InvalidConfiguration error"),
        }
    }

    #[test]
    fn test_sampler_validation_clamp_to_border_without_color() {
        let descriptor = SamplerDescriptor::new(None).with_address_mode(AddressMode::ClampToBorder);

        let result = descriptor.validate();
        assert!(result.is_err());
        match result {
            Err(SamplerError::InvalidConfiguration(msg)) => {
                assert!(msg.contains("border_color"));
            }
            _ => panic!("Expected InvalidConfiguration error"),
        }
    }

    #[test]
    fn test_sampler_validation_clamp_to_border_with_color() {
        let descriptor = SamplerDescriptor::new(None)
            .with_address_mode(AddressMode::ClampToBorder)
            .with_border_color(wgpu::SamplerBorderColor::TransparentBlack);

        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn test_sampler_validation_success() {
        let descriptor = SamplerDescriptor::new(None)
            .with_address_mode(AddressMode::Repeat)
            .with_filter(FilterMode::Linear)
            .with_lod_clamp(0.0, 10.0)
            .with_anisotropy(8);

        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn test_error_display() {
        let err = SamplerError::InvalidConfiguration("test error".to_string());
        assert_eq!(err.to_string(), "Invalid sampler configuration: test error");
    }

    #[test]
    fn test_complex_sampler_configuration() {
        // Test a complex sampler configuration with all features
        let descriptor = SamplerDescriptor::new(Some("complex_sampler"))
            .with_address_mode_u(AddressMode::Repeat)
            .with_address_mode_v(AddressMode::MirrorRepeat)
            .with_address_mode_w(AddressMode::ClampToEdge)
            .with_mag_filter(FilterMode::Linear)
            .with_min_filter(FilterMode::Linear)
            .with_mipmap_filter(MipmapFilterMode::Linear)
            .with_lod_clamp(0.0, 16.0)
            .with_anisotropy(16);

        assert_eq!(descriptor.label(), Some("complex_sampler"));
        assert_eq!(descriptor.address_mode_u(), AddressMode::Repeat);
        assert_eq!(descriptor.address_mode_v(), AddressMode::MirrorRepeat);
        assert_eq!(descriptor.address_mode_w(), AddressMode::ClampToEdge);
        assert_eq!(descriptor.mag_filter(), FilterMode::Linear);
        assert_eq!(descriptor.min_filter(), FilterMode::Linear);
        assert_eq!(descriptor.mipmap_filter(), MipmapFilterMode::Linear);
        assert_eq!(descriptor.lod_min_clamp(), 0.0);
        assert_eq!(descriptor.lod_max_clamp(), 16.0);
        assert_eq!(descriptor.anisotropy_clamp(), 16);
        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn test_comparison_sampler() {
        // Test a sampler for depth/stencil comparison
        let descriptor = SamplerDescriptor::new(Some("depth_sampler"))
            .with_address_mode(AddressMode::ClampToEdge)
            .with_filter(FilterMode::Linear)
            .with_compare(CompareFunction::LessEqual);

        assert_eq!(descriptor.label(), Some("depth_sampler"));
        assert_eq!(descriptor.compare(), Some(CompareFunction::LessEqual));
        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn test_default_sampler() {
        let descriptor = SamplerDescriptor::default();

        assert_eq!(descriptor.label(), None);
        assert_eq!(descriptor.address_mode_u(), AddressMode::ClampToEdge);
        assert_eq!(descriptor.mag_filter(), FilterMode::Nearest);
        assert_eq!(descriptor.lod_min_clamp(), 0.0);
        assert_eq!(descriptor.anisotropy_clamp(), 1);
        assert!(descriptor.validate().is_ok());
    }
}
