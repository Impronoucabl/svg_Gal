use svg::Document;
use svg::node::element::path::Data;
use svg::node::element::{SVG, Circle, Path};

use crate::gall_struct::{GallWord, Decor, GallCircle};
use crate::gall_ord::GallOrd;

pub struct GallPhrase {
    pub words:Vec<GallWord>,
    pub radius:f64,
}
impl GallPhrase {
    pub fn dash_list(&self) -> (usize, Vec<(usize,(usize,usize))>) {
        let mut dashes = Vec::new();
        let mut word_index = 0;
        let mut count:usize = 0;
        for word in &self.words {
            for dash in word.collect_dashes() {
                dashes.push((word_index, dash));
                count += 1;
            }
            word_index += 1;
        }
        (count, dashes)
    }
    pub fn dash_pair_loop_step(&mut self) -> (usize, Vec<(usize,(usize,usize))>) {
        let (_, list_dash) = self.dash_list();
        let mut spare_dash = Vec::new(); 
        let mut dashes = list_dash.into_iter();
        let mut count = 0;
        loop {
            let (word1, addr1) = match dashes.next() {
                Some(dec) => dec,
                None => break,
            };
            let (word2, addr2) = match dashes.next() {
                Some(dec) => dec,
                None => {spare_dash.push((word1,addr1)); count += 1; break},
            };
            if word1 == word2 && addr1.0 == addr2.0 {
                //reverse the order
                spare_dash.push((word2,addr2));
                spare_dash.push((word1,addr1));
                count += 2;
                println!("Spare!");
                continue
            }
            let address_1 = (word1,addr1.0,addr1.1);
            let address_2 = (word2,addr2.0,addr2.1);
            {
                let dash1 = self.get_mut_dash(address_1);
                dash1.add_syl_pair(address_2);
                dash1.free = false;
            }
            {
                let dash2 = self.get_mut_dash(address_2);
                dash2.add_syl_pair(address_1);
                dash2.free = false;
            }
        }
        (count, spare_dash)
    }
    pub fn get_dash_svg_xy(&self, addr:(usize,usize,usize)) -> (f64,f64) {
        self.words[addr.0].syllables[addr.1].decorators[addr.2].loc.svg_ord()
    }
    pub fn get_mut_dash(&mut self, address:(usize,usize,usize)) ->  &mut Decor {
        &mut self.words[address.0].syllables[address.1].decorators[address.2]
    }
    fn get_syl(&self, address:(usize,usize)) -> &GallCircle {
        &self.words[address.0].syllables[address.1]
    }
    pub fn render(&self, mut svg_doc:Document, origin: GallOrd) -> SVG {
        for word in &self.words {
            svg_doc = word.render(svg_doc);
        }
        for word in &self.words {
            for letter in &word.syllables {
                for decor in &letter.decorators {
                    if decor.dot {
                        continue
                    }
                    let destination = match decor.pair_syllable {
                        Some(addr) =>self.get_dash_svg_xy(addr),
                        None => (300.0,300.0),
                    }; //sentence.get_dash_ord(addr)
                    let line_path = Data::new()
                        .move_to(decor.loc.svg_ord())
                        .line_to(destination);
                    let dash = Path::new()
                        .set("fill", "none")
                        .set("stroke", "black")
                        .set("stroke-width", 1)
                        .set("d", line_path);
                    svg_doc = svg_doc.add(dash);
                }
            }
        }
        let circle = Circle::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 6)
            .set("cx", origin.svg_x())
            .set("cy", origin.svg_y())
            .set("r", self.radius);
        svg_doc.add(circle)
    }

}