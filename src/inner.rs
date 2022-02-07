use rand::seq::IteratorRandom;
use colored::*;

lazy_static::lazy_static! {
    static ref CWD: std::path::PathBuf = dirs::home_dir().unwrap().join(".redant");
}

pub struct Elements {
    name: String,
    data: String,
    origin: String,
}

pub trait Basic {
    fn new(name: &str, origin: &str) -> Self;
    fn read_cwd_or_origin(&mut self);
    fn write_to_home(&self);
    fn count(&self) -> usize;
    fn random(&self) -> String;
    fn filter_on_len(&self, len: usize) -> Vec<&str>;
    fn check_print_add(&mut self, list_elem: Vec<&str>);
}

impl Basic for Elements {
    fn new(name: &str, origin: &str) -> Self {
        Self {
            name: name.into(),
            origin: origin.into(),
            data: "".into(),
        }
    }
    fn read_cwd_or_origin(&mut self) {
        self.data = std::fs::read_to_string(&*CWD.join(&self.name)).unwrap_or_else(|_| {
            self.write_to_home();
            (&self.origin).into()
        });
    }

    fn write_to_home(&self) {
        std::fs::create_dir_all(&*CWD).unwrap();
        std::fs::write(&*CWD.join(&self.name), &self.origin).unwrap();
    }

    fn count(&self) -> usize {
        // println!("{:?}", self.data);
        self.data.lines().count()
    }
    
    fn random(&self) -> String {
        self.data.lines().choose(&mut rand::thread_rng()).unwrap().into()
    }

    fn filter_on_len(&self, len: usize) -> Vec<&str> {
        self.data.lines().filter(|elem|{
            elem.len()==len
        }).collect::<Vec<&str>>()
    }
    
    fn check_print_add(&mut self, list_elem: Vec<&str>) {
        // let mut return_string = "".to_owned();
        let mut return_string = self.data.to_owned();
        for elem in list_elem {
            if return_string.lines().any(|oneline| oneline == elem) {
                print!("{} ", elem.red());
            } else {
                print!("{} ", elem.green());
                return_string.push_str(&format!("\n{}", &elem))
            }
        }
        std::fs::write(&*CWD.join(&self.name), return_string).unwrap();
    }
} 

pub struct Composition<'a> {
    container: Vec<&'a Elements>,
}

// impl<'a> Composition<'a> {
//     fn new(arr: Vec<&'a Object>) -> Self {
//         Self {container: arr}
//     }
// }

pub trait Advanced<'a> {
    fn new(arr: Vec<&'a Elements>) -> Self;
    fn count(&self) -> usize;
    fn random(&self) -> String;
}

impl<'a> Advanced<'a> for Composition<'a> {
    fn new(arr: Vec<&'a Elements>) -> Self {
        Self {container: arr}
    }

    fn count(&self) -> usize {
        // self.container.product()
        let mut return_value: usize = 1usize;
        for element in &self.container {
            return_value *= element.count();
        }
        return_value
    }

    fn random(&self) -> String {
        let mut return_value = "".to_owned();
        for element in &self.container {
            return_value.push_str(&element.random())
        }
        return_value
    }

}
