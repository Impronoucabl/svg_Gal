use std::f64::consts::PI;

use svg::Document;
use svg::node::element::{Path, Circle, SVG};
use svg::node::element::path::Data;

use crate::gall_circle::{Circle as GCircle, HollowCircle};
use crate::gall_fn;
use crate::gall_loc::Location;
use crate::gall_stem::Stem;
use crate::gall_tainer::GallTainer;
use crate::gall_word::GallWord;

pub trait renderable {
    fn render(self, drawn:Document) -> Document;
}

impl renderable for GallWord {
    fn render(self, mut drawn:Document) -> Document {
        for tainer in self.tainer_vec {
            drawn = tainer.render(drawn);
        }
        drawn
    }
}

impl renderable for GallTainer {
    fn render(self, mut drawn:Document) -> Document {
        for stem in self.stem {
            drawn = stem.render(drawn);
        }
        drawn
    }
}

impl renderable for Stem {
    fn render(self, drawn:Document) -> Document {
        let circle = Circle::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", self.thick())
            .set("cx", self.x())
            .set("cy", self.y())
            .set("r", self.radius());
        drawn.add(circle)
    }
}