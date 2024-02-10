pub trait Collapsible
where Self: Sized
{
    type Type;

    fn possiblilities(&mut self, combine: &[Self::Type]) -> Result<&[Self::Type], CollapseError>;
    fn texture(&self) -> Texture;
}

pub enum CollapseError {
    TryToCollapseSingleOption,
    NoOptionsLeft,
}

pub enum Texture {}

