use derive_setters::Setters;
use glam::vec2;
use crate::{
  draw::{ImageHandle, RoundedCorners, UiDrawCommand},
  element::{MeasureContext, ProcessContext, UiElement},
  layout::{compute_size, Size, Size2d},
  measure::Response,
  rect::{Corners, FillColor},
};

#[derive(Setters)]
#[setters(prefix = "with_")]
pub struct Image {
  /// Image handle to draw
  #[setters(skip)]
  pub image: ImageHandle,

  /// Size of the image.
  ///
  /// - If one of the dimensions is `Size::Auto`, the image will be scaled to fit the other dimension\
  ///   (aspect ratio is preserved)
  /// - If both dimensions are `Size::Auto`, the image will be drawn at its original size
  /// - All other values behave as expected
  #[setters(into)]
  pub size: Size2d,

  /// Color of the image
  ///
  /// Image will get multiplied/tinted by this color or gradient
  #[setters(into)]
  pub color: FillColor,

  /// Corner radius of the image
  #[setters(into)]
  pub corner_radius: Corners<f32>,
}

impl Image {
  pub fn new(handle: ImageHandle) -> Self {
    Self {
      image: handle,
      size: Size2d {
        width: Size::Auto,
        height: Size::Auto,
      },
      color: (1., 1., 1.).into(),
      corner_radius: Corners::all(0.),
    }
  }
}

impl UiElement for Image {
  fn name(&self) -> &'static str {
    "image"
  }

  fn size(&self) -> Option<Size2d> {
    Some(self.size)
  }

  fn measure(&self, ctx: MeasureContext) -> Response {
    let dim = ctx.images.get_size(self.image).expect("invalid image handle");
    let pre_size = compute_size(ctx.layout, self.size, dim.as_vec2());
    Response {
      size: compute_size(ctx.layout, self.size, vec2(
        match self.size.height {
          Size::Auto => dim.x as f32,
          _ => (pre_size.y / dim.y as f32) * dim.x as f32,
        },
        match self.size.height {
          Size::Auto => dim.x as f32,
          _ => (pre_size.y / dim.y as f32) * dim.x as f32,
        },
      )),
      ..Default::default()
    }
  }

  fn process(&self, ctx: ProcessContext) {
    if !self.color.is_transparent() {
      ctx.draw.add(UiDrawCommand::Rectangle {
        position: ctx.layout.position,
        size: ctx.measure.size,
        color: self.color.corners(),
        texture: Some(self.image),
        texture_uv: None,
        rounded_corners: (self.corner_radius.max_f32() > 0.).then_some({
          RoundedCorners::from_radius(self.corner_radius)
        }),
      });
    }
  }
}
