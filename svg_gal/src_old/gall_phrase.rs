use crate::gall_struct::{GallWord, Decor, GallCircle};

pub struct CircleGallLine {
    dash_a:(usize,usize,usize),
    dash_b:(usize,usize,usize),
    thickness:f64,
    radius:f64,
    inner_radius:f64,
    outer_radius:f64,
}

impl CircleGallLine {
    //creates a new CircleGallLine. Besure to externally mark dashes as not free
    pub fn new(dash_a:(usize,usize,usize), dash_b:(usize,usize,usize), thickness:f64, straight: bool) -> CircleGallLine {
        let radius:f64;
        if straight {
            radius = f64::INFINITY;
        } else {
            radius = 30.0;
        }
        CircleGallLine { 
            dash_a, 
            dash_b, 
            thickness:thickness, 
            radius:radius,
            inner_radius: radius - thickness,
            outer_radius: radius + thickness,
        }
    }
    pub fn straight(&self) -> bool {
        self.radius.is_infinite()
    }
    pub fn get_source(&self, sentence:&GallPhrase) -> (f64,f64) {
        sentence.get_dash_svg_xy(self.dash_a)
    }
    pub fn get_target(&self, sentence:&GallPhrase) -> (f64,f64) {
        sentence.get_dash_svg_xy(self.dash_b)
    }
    pub fn get_thickness(&self) -> f64 {
        self.thickness
    }

}

pub struct GallPhrase {
    pub words:Vec<GallWord>,
    pub dash_pairs: Vec<CircleGallLine>,
    pub radius:f64,
    pub thickness: f64,
}

impl GallPhrase {
    fn pair_dash(&mut self, address_1: (usize, usize, usize), address_2: (usize, usize, usize)) -> CircleGallLine {
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
        CircleGallLine::new(address_1, address_2, 1.0, true)
    }
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
    pub fn dash_pair_loop_step(&mut self, spare:Option<Vec<(usize,(usize,usize))>>) -> (usize, Option<Vec<(usize,(usize,usize))>>) {
        let (_, list_dash) = match  spare {
            Some(list) => (0,list),
            None => self.dash_list(),
        };
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
            if !(word1 == word2 && addr1.0 == addr2.0) {
                let address_1 = (word1,addr1.0,addr1.1);
                let address_2 = (word2,addr2.0,addr2.1);
                let pair = self.pair_dash(address_1, address_2);
                self.dash_pairs.push(pair);
            } else {
                let (word3, addr3) = match dashes.next() {
                    Some(dec) => dec,
                    None => {
                        let mut front = vec![(word2,addr2)];
                        front.append(&mut spare_dash);
                        spare_dash = front;
                        spare_dash.push((word1,addr1)); 
                        count += 2; 
                        break
                    },
                };
                if word3 == word1 && addr3.0 == addr1.0 {
                    spare_dash.push((word3,addr3));
                    spare_dash.push((word2,addr2));
                    spare_dash.push((word1,addr1));
                    count += 3;
                    println!("Spare!");
                    continue
                }
                let address_1 = (word1,addr1.0,addr1.1);
                let address_2 = (word3,addr3.0,addr3.1);
                let pair = self.pair_dash(address_1, address_2);
                self.dash_pairs.push(pair);
                spare_dash.push((word2,addr2));
                count += 1;
            }  
        }
        if spare_dash.is_empty() {
            (count, None)
        } else {
            (count, Some(spare_dash))
        }
    }
    pub fn dash_pair_loop(&mut self) {
        let mut total_dash = 0;
        for word in &self.words {
            total_dash += 5*word.letter_count;
        }
        let (mut count, mut spare) = self.dash_pair_loop_step(None);
        while total_dash > count {
            total_dash = count;
            (count, spare) = self.dash_pair_loop_step(spare);
        }

    }
    pub fn dock_words(&mut self) {
        println!("docking");
        for word in &self.words {
            let dock_list = word.collect_t_stem();
            println!("{}", dock_list.len())
        }
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
}