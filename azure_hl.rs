// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! High-level bindings to Azure.

use azure::{AZ_CAP_BUTT, AZ_JOIN_MITER_OR_BEVEL};
use azure::{AzPoint, AzRect, AzFloat, AzIntSize, AzColor, AzColorPatternRef};
use azure::{AzStrokeOptions, AzDrawOptions, AzSurfaceFormat, AzFilter, AzDrawSurfaceOptions};
use azure::{AzBackendType, AzDrawTargetRef, AzSourceSurfaceRef, AzDataSourceSurfaceRef};
use azure::{AzScaledFontRef, AzGlyphRenderingOptionsRef};
use azure::{struct__AzColor, struct__AzGlyphBuffer};
use azure::{struct__AzDrawOptions, struct__AzDrawSurfaceOptions, struct__AzIntSize};
use azure::{struct__AzPoint, struct__AzRect, struct__AzStrokeOptions};
use azure::{AzGLContext, AzSkiaSharedGLContextRef};
use azure::{AzCreateColorPattern, AzCreateDrawTarget, AzCreateDrawTargetForData};
use azure::{AzDataSourceSurfaceGetData, AzDataSourceSurfaceGetStride};
use azure::{AzDrawTargetClearRect};
use azure::{AzDrawTargetCreateSourceSurfaceFromData, AzCreateSkiaSharedGLContext};
use azure::{AzReleaseSkiaSharedGLContext, AzRetainSkiaSharedGLContext};
use azure::{AzDrawTargetDrawSurface, AzDrawTargetFillRect, AzDrawTargetFlush};
use azure::{AzDrawTargetGetSize, AzDrawTargetGetSnapshot, AzDrawTargetSetTransform};
use azure::{AzDrawTargetStrokeLine, AzDrawTargetStrokeRect, AzDrawTargetFillGlyphs};
use azure::{AzReleaseColorPattern, AzReleaseDrawTarget};
use azure::{AzReleaseSourceSurface, AzRetainDrawTarget};
use azure::{AzSourceSurfaceGetDataSurface, AzSourceSurfaceGetFormat};
use azure::{AzSourceSurfaceGetSize, AzCreateSkiaDrawTargetForFBO, AzSkiaGetCurrentGLContext};
use azure::{AzSkiaSharedGLContextMakeCurrent, AzSkiaSharedGLContextStealSurface};
use azure::{AzSkiaSharedGLContextFlush, AzSkiaGrGLSharedSurfaceRef};
use azure::{AzCreatePathBuilder, AzPathBuilderRef, AzPathBuilderMoveTo, AzPathBuilderLineTo, AzPathBuilderFinish, AzReleasePathBuilder};
use azure::{AzDrawTargetFill, AzPathRef, AzReleasePath, AzDrawTargetPushClip, AzDrawTargetPopClip};

use sync::Arc;
use geom::matrix2d::Matrix2D;
use geom::point::Point2D;
use geom::rect::Rect;
use geom::size::Size2D;
use layers::platform::surface::{NativeGraphicsMetadata, NativePaintingGraphicsContext};
use libc::types::common::c99::{uint8_t, uint16_t};
use libc::size_t;
use std::mem;
use std::ptr;
use std::slice;

#[cfg(target_os="linux")]
use libc::c_void;

pub trait AsAzureRect {
    fn as_azure_rect(&self) -> AzRect;
}

impl AsAzureRect for Rect<AzFloat> {
    fn as_azure_rect(&self) -> AzRect {
        struct__AzRect {
            x: self.origin.x,
            y: self.origin.y,
            width: self.size.width,
            height: self.size.height
        }
    }
}

pub trait AsAzureIntSize {
    fn as_azure_int_size(&self) -> AzIntSize;
}

impl AsAzureIntSize for Size2D<i32> {
    fn as_azure_int_size(&self) -> AzIntSize {
        struct__AzIntSize {
            width: self.width,
            height: self.height
        }
    }
}

pub trait AsAzurePoint {
    fn as_azure_point(&self) -> AzPoint;
}

impl AsAzurePoint for Point2D<AzFloat> {
    fn as_azure_point(&self) -> AzPoint {
        struct__AzPoint {
            x: self.x,
            y: self.y
        }
    }
}

#[deriving(Clone)]
pub struct Color {
    pub r: AzFloat,
    pub g: AzFloat,
    pub b: AzFloat,
    pub a: AzFloat,
}

impl Color {
    pub fn new(r: AzFloat, g: AzFloat, b: AzFloat, a: AzFloat) -> Color {
        Color { r: r, g: g, b: b, a: a }
    }

    fn as_azure_color(&self) -> AzColor {
        struct__AzColor { r: self.r, g: self.g, b: self.b, a: self.a }
    }
}


// FIXME: Should have a class hierarchy here starting with Pattern.
pub struct ColorPattern {
    pub azure_color_pattern: AzColorPatternRef,
}

impl Drop for ColorPattern {
    fn drop(&mut self) {
        unsafe {
            AzReleaseColorPattern(self.azure_color_pattern);
        }
    }
}

impl ColorPattern {
    pub fn new(color: Color) -> ColorPattern {
        unsafe {
            ColorPattern {
                azure_color_pattern: AzCreateColorPattern(&mut color.as_azure_color())
            }
        }
    }
}

pub enum CompositionOp {
    OverOp,
    AddOp,
    AtopOp,
    OutOp,
    InOp,
    SourceOp,
    DestInOp,
    DestOutOp,
    DestOverOp,
    DestAtopOp,
    XorOp,
    MultiplyOp,
    ScreenOp,
    OverlayOp,
    DarkenOp,
    LightenOp,
    ColorDodgeOp,
    ColorBurnOp,
    HardLightOp,
    SoftLightOp,
    DifferenceOp,
    ExclusionOp,
    HueOp,
    SaturationOp,
    ColorOp,
    LuminosityOp,
}

pub struct StrokeOptions {
    pub line_width: AzFloat,
    pub miter_limit: AzFloat,
    pub mDashPattern: *mut AzFloat,
    pub mDashLength: size_t,
    pub fields: uint8_t
}

impl StrokeOptions {
    pub fn new(line_width: AzFloat, miter_limit: AzFloat) -> StrokeOptions {
        StrokeOptions {
            line_width: line_width,
            miter_limit: miter_limit,
            mDashPattern: ptr::mut_null(),
            mDashLength: 0,
            fields: AZ_CAP_BUTT as u8 << 4 | AZ_JOIN_MITER_OR_BEVEL as u8
        }
    }

    fn as_azure_stroke_options(&self) -> AzStrokeOptions {
        struct__AzStrokeOptions {
            mLineWidth: self.line_width,
            mMiterLimit: self.miter_limit,
            mDashPattern: self.mDashPattern,
            mDashLength: self.mDashLength,
            mDashOffset: 0.0 as AzFloat,
            fields: self.fields
        }
    }

    pub fn set_join_style(&mut self, style: u8) {
        self.fields = self.fields & 0b1111_0000_u8;
        self.fields = self.fields | style ;
    }

    pub fn set_cap_style(&mut self, style: u8) {
        self.fields = self.fields & 0b0000_1111_u8;
        self.fields = self.fields | (style << 4);
    }
}

pub struct DrawOptions {
    pub alpha: AzFloat,
    pub fields: uint16_t,
}

impl DrawOptions {
    pub fn new(alpha: AzFloat, fields: uint16_t) -> DrawOptions {
        DrawOptions {
            alpha : alpha,
            fields : fields,
        }
    }

    fn as_azure_draw_options(&self) -> AzDrawOptions {
        struct__AzDrawOptions {
            mAlpha: self.alpha,
            fields: self.fields
        }
    }

    pub fn set_composition_op(&mut self, style: CompositionOp) {
        self.fields = self.fields & 0b1111_1111_0000_0000_u16;
        self.fields = self.fields | (style as u16);
    }

    pub fn set_antialias_mode(&mut self, style: u8) {
        self.fields = self.fields & 0b1111_1000_1111_1111_u16; 
        let style = ((style & 7) as u16) << 8;
        self.fields = self.fields | style;
    }

    pub fn set_snapping(&mut self, style: u8) {
        self.fields = self.fields & 0b1111_0111_1111_1111_u16;
        let style = ((style & 1) as u16) << 11;
        self.fields = self.fields | style;
    }
}


pub enum SurfaceFormat {
    B8G8R8A8,
    B8G8R8X8,
    R5G6B5,
    A8
}

impl SurfaceFormat {
    fn as_azure_surface_format(self) -> AzSurfaceFormat {
        self as AzSurfaceFormat
    }

    pub fn new(azure_surface_format: AzSurfaceFormat) -> SurfaceFormat {
        match azure_surface_format {
            0 => B8G8R8A8,
            1 => B8G8R8X8,
            2 => R5G6B5,
            3 => A8,
            _ => fail!("SurfaceFormat::new(): unknown Azure surface format")
        }
    }
}

pub enum Filter {
    Linear,
    Point
}

impl Filter {
    pub fn as_azure_filter(self) -> AzFilter {
        self as AzFilter
    }
}

pub struct DrawSurfaceOptions {
    pub filter: Filter,
    pub sampling_bounds: bool,
}

impl DrawSurfaceOptions {
    pub fn new(filter: Filter, sampling_bounds: bool) -> DrawSurfaceOptions {
        DrawSurfaceOptions {
            filter: filter,
            sampling_bounds: sampling_bounds,
        }
    }

    fn as_azure_draw_surface_options(&self) -> AzDrawSurfaceOptions {
        struct__AzDrawSurfaceOptions {
            fields: ((self.filter as int) | (if self.sampling_bounds { 8 } else { 0 })) as u32
        }
    }
}


#[deriving(Clone, PartialEq)]
pub enum BackendType {
    NoBackend,
    Direct2DBackend,
    CoreGraphicsBackend,
    CoreGraphicsAcceleratedBackend,
    CairoBackend,
    SkiaBackend,
    RecordingBackend
}

impl BackendType {
    pub fn as_azure_backend_type(self) -> AzBackendType {
        match self {
            NoBackend                      => 0,
            Direct2DBackend                => 1,
            CoreGraphicsBackend            => 2,
            CoreGraphicsAcceleratedBackend => 3,
            CairoBackend                   => 4,
            SkiaBackend                    => 5,
            RecordingBackend               => 6,
        }
    }
}

pub struct DrawTarget {
    pub azure_draw_target: AzDrawTargetRef,
    pub data: Option<Arc<Vec<u8>>>,
    pub skia_context: Option<AzSkiaSharedGLContextRef>
}

impl Drop for DrawTarget {
    fn drop(&mut self) {
        unsafe {
            AzReleaseDrawTarget(self.azure_draw_target);
            match self.skia_context {
                None => {}
                Some(ctx_ref) => { AzReleaseSkiaSharedGLContext(ctx_ref); }
            }
        }
    }
}

/// Contains the GL resources that Skia was holding onto that may be safely extracted. At the
/// moment this consists simply of the native surface.
pub struct StolenGLResources {
    pub surface: AzSkiaGrGLSharedSurfaceRef,
}

impl DrawTarget {
    pub fn new(backend: BackendType, size: Size2D<i32>, format: SurfaceFormat)
                   -> DrawTarget {
        unsafe {
            let azure_draw_target = AzCreateDrawTarget(backend.as_azure_backend_type(),
                                                       &mut size.as_azure_int_size(),
                                                       format.as_azure_surface_format());
            if azure_draw_target == ptr::mut_null() { fail!("null azure draw target"); }
            DrawTarget {
                azure_draw_target: azure_draw_target,
                data: None,
                skia_context: None
            }
        }
    }

    pub fn new_with_data(backend: BackendType,
                         mut data: Vec<u8>,
                         offset: uint,
                         size: Size2D<i32>,
                         stride: i32,
                         format: SurfaceFormat) -> DrawTarget {
        unsafe {
            assert!((data.len() - offset) as i32 >= stride * size.height);
            let azure_draw_target =
                AzCreateDrawTargetForData(backend.as_azure_backend_type(),
                                          data.get_mut(offset),
                                          &mut size.as_azure_int_size(),
                                          stride,
                                          format.as_azure_surface_format());
            if azure_draw_target == ptr::mut_null() { fail!("null azure draw target"); }
            DrawTarget {
                azure_draw_target: azure_draw_target,
                data: Some(Arc::new(data)),
                skia_context: None
            }
        }
    }

    pub fn new_with_fbo(backend: BackendType,
                        native_graphics_context: &NativePaintingGraphicsContext,
                        size: Size2D<i32>,
                        format: SurfaceFormat) -> DrawTarget {
        assert!(backend == SkiaBackend);
        unsafe {
            let native_graphics_context = mem::transmute(native_graphics_context);
            let skia_context = AzCreateSkiaSharedGLContext(native_graphics_context,
                                                           &mut size.as_azure_int_size());
            let azure_draw_target = AzCreateSkiaDrawTargetForFBO(skia_context,
                                                                 &mut size.as_azure_int_size(),
                                                                 format.as_azure_surface_format());
            if azure_draw_target == ptr::mut_null() {
                fail!("null azure draw target");
            }
            DrawTarget {
                azure_draw_target: azure_draw_target,
                data: None,
                skia_context: Some(skia_context)
            }
        }
    }

    pub fn clone(&self) -> DrawTarget {
        unsafe {
            AzRetainDrawTarget(self.azure_draw_target);
            if self.skia_context.is_some() {
                AzRetainSkiaSharedGLContext(self.skia_context.unwrap());
            }
        }
        DrawTarget {
            azure_draw_target: self.azure_draw_target,
            data: match self.data {
                None => None,
                Some(ref arc) => Some(arc.clone())
            },
            skia_context: self.skia_context
                
        }
    }

    pub fn make_current(&self) {
        match self.skia_context {
            None => {}
            Some(ctx) => { 
                unsafe {
                    AzSkiaSharedGLContextMakeCurrent(ctx);
                }
            }
        }
    }

    /// Consumes this draw target and returns the underlying native surface and GL context, if they exist.
    pub fn steal_gl_resources(self) -> Option<StolenGLResources> {
        match self.skia_context {
            None => None,
            Some(ctx) => {
                unsafe {
                    Some(StolenGLResources {
                        surface: AzSkiaSharedGLContextStealSurface(ctx),
                    })
                }
            }
        }
    }

    pub fn get_size(&self) -> AzIntSize {
        unsafe {
            AzDrawTargetGetSize(self.azure_draw_target)
        }
    }

    pub fn flush(&self) {
        unsafe {
            AzDrawTargetFlush(self.azure_draw_target);
            match self.skia_context {
                None => {}
                Some(ctx) => { AzSkiaSharedGLContextFlush(ctx); }
            }
        }
    }

    pub fn clear_rect(&self, rect: &Rect<AzFloat>) {
        unsafe {
            AzDrawTargetClearRect(self.azure_draw_target, &mut rect.as_azure_rect());
        }
    }

    pub fn fill(&self, path: &Path, pattern: &ColorPattern, draw_options: &DrawOptions) {
        unsafe {
            AzDrawTargetFill(self.azure_draw_target,
                             path.azure_path,
                             pattern.azure_color_pattern,
                             &mut draw_options.as_azure_draw_options());
        }
    }

    pub fn fill_rect(&self,
                     rect: &Rect<AzFloat>,
                     pattern: &ColorPattern,
                     draw_options: Option<&DrawOptions>) {
        unsafe {
            let draw_options = draw_options.map(|draw_options| {
                draw_options.as_azure_draw_options()
            });
            let draw_options = match draw_options {
                None => ptr::mut_null(),
                Some(mut draw_options) => {
                    let draw_options: *mut AzDrawOptions = &mut draw_options;
                    draw_options
                }
            };
            AzDrawTargetFillRect(self.azure_draw_target,
                                 &mut rect.as_azure_rect(),
                                 pattern.azure_color_pattern,
                                 draw_options);
        }
    }

    pub fn stroke_line(&self,
                   start: Point2D<AzFloat>,
                   end: Point2D<AzFloat>,
                   pattern: &ColorPattern,
                   stroke_options: &StrokeOptions,
                   draw_options: &DrawOptions) {
        unsafe {
            AzDrawTargetStrokeLine(self.azure_draw_target,
                                   &mut start.as_azure_point(),
                                   &mut end.as_azure_point(),
                                   pattern.azure_color_pattern,
                                   &mut stroke_options.as_azure_stroke_options(),
                                   &mut draw_options.as_azure_draw_options());
        }
    }

    pub fn stroke_rect(&self,
                   rect: &Rect<AzFloat>,
                   pattern: &ColorPattern,
                   stroke_options: &StrokeOptions,
                   draw_options: &DrawOptions) {
        unsafe {
            AzDrawTargetStrokeRect(self.azure_draw_target,
                                   &mut rect.as_azure_rect(),
                                   pattern.azure_color_pattern,
                                   &mut stroke_options.as_azure_stroke_options(),
                                   &mut draw_options.as_azure_draw_options());
        }
    }

    pub fn draw_surface(&self,
                    surface: SourceSurface,
                    dest: Rect<AzFloat>,
                    source: Rect<AzFloat>,
                    surf_options: DrawSurfaceOptions,
                    options: DrawOptions) {
        unsafe {
            AzDrawTargetDrawSurface(self.azure_draw_target,
                                    surface.azure_source_surface,
                                    &mut dest.as_azure_rect(),
                                    &mut source.as_azure_rect(),
                                    &mut surf_options.as_azure_draw_surface_options(),
                                    &mut options.as_azure_draw_options());
        }
    }

    pub fn snapshot(&self) -> SourceSurface {
        unsafe {
            let azure_surface = AzDrawTargetGetSnapshot(self.azure_draw_target);
            SourceSurface::new(azure_surface)
        }
    }

    pub fn create_source_surface_from_data(&self,
                                       data: &[u8],
                                       size: Size2D<i32>,
                                       stride: i32,
                                       format: SurfaceFormat)
                                    -> SourceSurface {
        assert!(data.len() as i32 == stride * size.height);
        unsafe {
            let azure_surface = AzDrawTargetCreateSourceSurfaceFromData(
                self.azure_draw_target,
                &data[0],
                &mut size.as_azure_int_size(),
                stride,
                format.as_azure_surface_format());
            SourceSurface::new(azure_surface)
        }
    }

    pub fn set_transform(&self, matrix: &Matrix2D<AzFloat>) {
        unsafe {
            AzDrawTargetSetTransform(self.azure_draw_target, mem::transmute(matrix));
        }
    }

    pub fn fill_glyphs(&self,
                       azfontref: AzScaledFontRef,
                       mut glyphbuf: struct__AzGlyphBuffer,
                       azure_pattern: AzColorPatternRef,
                       mut options: struct__AzDrawOptions,
                       renderingOptions: AzGlyphRenderingOptionsRef) {
        unsafe {
            AzDrawTargetFillGlyphs(self.azure_draw_target,
                    azfontref,
                    &mut glyphbuf,
                    azure_pattern,
                    &mut options,
                    renderingOptions);
        }
    }

    pub fn create_path_builder(&self) -> PathBuilder {
        unsafe {
            PathBuilder {
                azure_path_builder: AzCreatePathBuilder(self.azure_draw_target)
            }
        }
    }

    pub fn push_clip(&self, path: &Path) {
        unsafe {
            AzDrawTargetPushClip(self.azure_draw_target,path.azure_path);
        }
    }

    pub fn pop_clip(&self) {
        unsafe {
            AzDrawTargetPopClip(self.azure_draw_target);
        }

    }
}

// Ugly workaround for the lack of explicit self.
pub fn clone_mutable_draw_target(draw_target: &mut DrawTarget) -> DrawTarget {
    return draw_target.clone();
}

pub struct SourceSurface {
    pub azure_source_surface: AzSourceSurfaceRef,
}

impl Drop for SourceSurface {
    fn drop(&mut self) {
        unsafe {
            AzReleaseSourceSurface(self.azure_source_surface);
        }
    }
}

impl SourceSurface {
    pub fn new(azure_source_surface: AzSourceSurfaceRef) -> SourceSurface {
        SourceSurface {
            azure_source_surface: azure_source_surface
        }
    }
}

// FIXME Rust #8753 no fixed stack segment for default methods
unsafe fn AzSourceSurfaceGetSize_(aSurface: AzSourceSurfaceRef) -> AzIntSize {
    AzSourceSurfaceGetSize(aSurface)
}

// FIXME Rust #8753 no fixed stack segment for default methods
unsafe fn AzSourceSurfaceGetFormat_(aSurface: AzSourceSurfaceRef) -> AzSurfaceFormat {
    AzSourceSurfaceGetFormat(aSurface)
}

pub trait SourceSurfaceMethods {
    fn get_azure_source_surface(&self) -> AzSourceSurfaceRef;

    fn size(&self) -> Size2D<i32> {
        unsafe {
            let size = AzSourceSurfaceGetSize_(self.get_azure_source_surface());
            Size2D { width: size.width, height: size.height }
        }
    }

    fn format(&self) -> SurfaceFormat {
        unsafe {
            SurfaceFormat::new(AzSourceSurfaceGetFormat_(self.get_azure_source_surface()))
        }
    }
}

impl SourceSurface {
    pub fn get_data_surface(&self) -> DataSourceSurface {
        unsafe {
            let data_source_surface = AzSourceSurfaceGetDataSurface(
                self.azure_source_surface);
            DataSourceSurface {
                azure_data_source_surface: data_source_surface
            }
        }
    }
}

impl SourceSurfaceMethods for SourceSurface {
    fn get_azure_source_surface(&self) -> AzSourceSurfaceRef { self.azure_source_surface }
}

pub struct DataSourceSurface {
    pub azure_data_source_surface: AzDataSourceSurfaceRef,
}

impl Drop for DataSourceSurface {
    fn drop(&mut self) {
        unsafe {
            AzReleaseSourceSurface(self.azure_data_source_surface);
        }
    }
}

impl DataSourceSurface {
    pub fn with_data(&self, f: |&[u8]|) {
        unsafe {
            let buf = AzDataSourceSurfaceGetData(self.azure_data_source_surface) as *const u8;
            let len = self.stride() * self.size().height;
            slice::raw::buf_as_slice(buf, len as uint, f);
        }
    }

    pub fn stride(&self) -> i32 {
        unsafe {
            AzDataSourceSurfaceGetStride(self.azure_data_source_surface)
        }
    }

    // FIXME: Workaround for lack of working cross-crate default methods.
    pub fn get_size(&self) -> Size2D<i32> {
        self.size()
    }
}

impl SourceSurfaceMethods for DataSourceSurface {
    fn get_azure_source_surface(&self) -> AzSourceSurfaceRef {
        self.azure_data_source_surface
    }
}

pub struct Path {
    pub azure_path: AzPathRef
}

impl Drop for Path {
    fn drop(&mut self){
        unsafe {
            AzReleasePath(self.azure_path);
        }
    }
}

pub struct PathBuilder {
    pub azure_path_builder: AzPathBuilderRef
}

impl PathBuilder {
    pub fn move_to(&self, point: Point2D<AzFloat>) {
        unsafe {
            let mut az_point = point.as_azure_point();
            AzPathBuilderMoveTo(self.azure_path_builder, &mut az_point);
        }
    }

    pub fn line_to(&self, point: Point2D<AzFloat>) {
        unsafe {
            let mut az_point = point.as_azure_point();
            AzPathBuilderLineTo(self.azure_path_builder, &mut az_point);
        }
    }

    pub fn finish(&self) -> Path{
        unsafe {
            let az_path = AzPathBuilderFinish(self.azure_path_builder);
            Path {
                azure_path : az_path
            }
        }
    }
}

impl Drop for PathBuilder {
    fn drop(&mut self) {
        unsafe {
            AzReleasePathBuilder(self.azure_path_builder);
        }
    }
}

pub fn current_gl_context() -> AzGLContext {
    unsafe {
        AzSkiaGetCurrentGLContext()
    }
}

#[cfg(target_os="linux")]
pub fn current_display() -> *mut c_void {
    use glfw;
    unsafe {
        glfw::ffi::glfwGetX11Display()
    }
}

#[cfg(target_os="linux")]
pub fn current_graphics_metadata() -> NativeGraphicsMetadata {
    NativeGraphicsMetadata {
        display: current_display(),
    }
}

#[cfg(target_os="macos")]
pub fn current_graphics_metadata() -> NativeGraphicsMetadata {
    use opengles::cgl::{CGLGetCurrentContext, CGLGetPixelFormat};
    unsafe {
        NativeGraphicsMetadata {
            pixel_format: CGLGetPixelFormat(CGLGetCurrentContext()),
        }
    }
}

#[cfg(target_os="android")]
pub fn current_graphics_metadata() -> NativeGraphicsMetadata {
    use egl::egl::GetCurrentDisplay;
    NativeGraphicsMetadata {
        display: GetCurrentDisplay(),
    }
}
