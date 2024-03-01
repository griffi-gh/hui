use glam::{Affine2, Vec2};
use crate::{
  draw::UiDrawCommand, element::{MeasureContext, ProcessContext, UiElement}, measure::Response
};

pub struct Transformer {
  pub transform: Affine2,
  pub element: Box<dyn UiElement>,
}

impl Transformer {
  pub fn new(element: Box<dyn UiElement>) -> Self {
    Self {
      transform: Affine2::IDENTITY,
      element,
    }
  }

  pub fn translate(mut self, v: impl Into<Vec2>) -> Self {
    self.transform *= Affine2::from_translation(v.into());
    self
  }

  pub fn scale(mut self, v: impl Into<Vec2>) -> Self {
    self.transform *= Affine2::from_scale(v.into());
    self
  }

  pub fn rotate(mut self, radians: f32) -> Self {
    self.transform *= Affine2::from_angle(radians);
    self
  }
}

impl UiElement for Transformer {
  fn measure(&self, ctx: MeasureContext) -> Response {
    self.element.measure(ctx)
  }

  fn process(&self, ctx: ProcessContext) {
    ctx.draw.add(UiDrawCommand::PushTransform(self.transform));
    //This is stupid:
    self.element.process(ProcessContext {
      measure: ctx.measure,
      state: ctx.state,
      layout: ctx.layout,
      draw: ctx.draw,
      text_measure: ctx.text_measure,
      current_font: ctx.current_font,
    });
    ctx.draw.add(UiDrawCommand::PopTransform);
  }
}

pub trait ElementTransformExt {
  fn transform(self) -> Transformer;
}

impl<T: UiElement + 'static> ElementTransformExt for T {
  /// Wrap the element in a [`Transformer`]
  ///
  /// This allows you to apply various transformations to the element, such as translation, rotation, or scaling\
  /// Use sparingly, as this is an experimental feature and may not work as expected\
  /// Transform is applied around the center of the element's bounding box.
  fn transform(self) -> Transformer {
    Transformer::new(Box::new(self))
  }
}