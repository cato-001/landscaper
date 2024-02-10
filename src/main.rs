#![allow(unused)]

use core::slice::SlicePattern;

use collapse::{CollapseError, Collapsible};

mod collapse;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq, Eq)]
enum Landmass {
    DeepSea,
    Sea,
    Flatland,
    Hills,
    Mountains,
    ExtremMountains
}

trait Neighbours
where Self: Sized
{
    fn neighbours(&self) -> &'static [Self];
}

impl Neighbours for Landmass {
    fn neighbours(&self) -> &'static [Self] {
        match self {
            Self::Sea => &[Self::DeepSea, Self::Sea, Self::Flatland, Self::Hills],
            Self::DeepSea => &[Self::DeepSea, Self::DeepSea],
            Self::Flatland => &[Self::Sea, Self::Flatland, Self::Hills],
            Self::Hills => &[Self::Sea, Self::Flatland, Self::Hills, Self::Mountains],
            Self::Mountains => &[Self::Hills, Self::Mountains, Self::ExtremMountains],
            Self::ExtremMountains => &[Self::Mountains, Self::ExtremMountains],
        }
    }
}

enum Cell<Type> {
    Collapsed {
        value: Type
    },
    Superposition {
        values: Vec<Type>
    }
}

impl<Type> Cell<Type> {
    fn collapsed(value: Type) -> Self {
        Self::Collapsed { value }
    }

    fn superposition(values: Vec<Type>) -> Self {
        Self::Superposition { values }
    }
}

impl Collapsible for Cell<Landmass> {
    type Type = Landmass;

    fn possiblilities(&mut self, combine: &[Self::Type]) -> Result<&[Self::Type], CollapseError> {
        let Self::Superposition { values } = self else {
            return Err(CollapseError::TryToCollapseSingleOption)
        };
        values.retain(|item| combine.contains(item));
        if values.is_empty() {
            return Err(CollapseError::NoOptionsLeft);
        }
        Ok(values.as_slice())
    }

    fn texture(&self) -> collapse::Texture {
        todo!()
    }
}
