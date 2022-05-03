use image::{open, imageops};
use std::collections::HashMap;

pub fn show_one2(number: &String) -> Box<Vec<String>> {
    let nwidth = 20;
    let nheight = 30;
    let gap_value = 120 as u8;

    let mut imagepath = "".to_string();
    if number == ":" {
        let number = "maohao";
        imagepath = format!("C:\\Users\\yuanz\\ClionProjects\\countdown\\NumberSimple\\{}.png", number);
    } else {
        imagepath = format!("C:\\Users\\yuanz\\ClionProjects\\countdown\\NumberSimple\\{}.png", number);
    }

    let gray = open(imagepath).unwrap()
        .resize(nwidth, nheight, imageops::FilterType::Nearest)
        .into_luma8();

    let mut resultdata = Vec::new();
    for _index in 0..nheight {
        resultdata.push(vec![])
    }
    for (index, tempvalue) in gray.to_vec().iter().enumerate() {
        let vectorindex = index / nwidth as usize;

        if tempvalue > &gap_value {
            resultdata[vectorindex].push("*");
        } else {
            resultdata[vectorindex].push(" ");
        }
    }

    let mut showdata = vec![];
    for i in resultdata {
        let mut shows = "".to_string();
        for j in i {
            shows.push_str(j)
        }
        showdata.push(shows)
    }

    return Box::new(showdata);
}


pub fn show_label(number: &String) -> Box<Vec<String>> {
    let nwidth = 28 * 2;
    let nheight = 28;
    let gap_value = 120 as u8;

    let mut imagepath = "".to_string();
    if number == ":" {
        let number = "maohao";
        imagepath = format!("C:\\Users\\yuanz\\ClionProjects\\countdown\\NumberSimple\\{}.png", number);
    } else {
        imagepath = format!("C:\\Users\\yuanz\\ClionProjects\\countdown\\NumberSimple\\{}.png", number);
    }

    let gray = open(imagepath).unwrap()
        .resize(nwidth, nheight, imageops::FilterType::Nearest)
        .into_luma8();

    let mut resultdata = Vec::new();
    for _index in 0..nheight {
        resultdata.push(vec![])
    }
    for (index, tempvalue) in gray.to_vec().iter().enumerate() {
        let vectorindex = index / nwidth as usize;

        if tempvalue > &gap_value {
            resultdata[vectorindex].push("*");
        } else {
            resultdata[vectorindex].push(" ");
        }
    }

    let mut showdata = vec![];
    for i in resultdata {
        let mut shows = "".to_string();
        for j in i {
            shows.push_str(j)
        }
        showdata.push(shows)
    }

    return Box::new(showdata);
}


// add oop style


pub struct CountDown{
    SimpleWord: HashMap<&'static str, Box<Vec<String>>>,
}

impl CountDown {

    pub fn new() -> Self {
        let mut dict_result = HashMap::new();
        let biglist = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", ":", "day", "hou", "min", "sec"];
        let number = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", ":"];
        for templist in biglist {
            if number.contains(&templist) {
                let result = show_one2(&templist.to_string());
                dict_result.insert(templist, result);
            } else {
                let result = show_label(&templist.to_string());
                dict_result.insert(templist, result);
            }
        }

        return Self {
            SimpleWord: dict_result,
        };
    }
    pub fn check(&self) {
        let var ="2";
        let data = self.SimpleWord.get(var).unwrap();
        for i in data.iter() {
            println!("{}", i);
        }


    }
    pub fn showtotallabel(&self) {
        let temp_list = vec!["day", "hou", "min", "sec"];


        let mut final_vector = vec![];
        for _index in 0..28 {
            final_vector.push("".to_string());
        }

        for tempvalue in temp_list {
            // let simple_vector = *show_label(&tempvalue.to_string());
            let simple_vector =  self.SimpleWord.get(tempvalue).unwrap();


            let mut index = 0;
            for x in simple_vector.iter() {
                final_vector[index].push_str(&*x);
                index += 1;
            }
        }

        for temp in final_vector {
            println!("{}", temp)
        }
    }


    pub fn showtotaldate(&self, inputstr: &String) {
        // part 2
        let mut temp_list = Vec::new();
        // let mut tempcharacter = "20:23:56:56".to_string();
        let tempcharacter = inputstr;
        for index in 0..tempcharacter.len() {
            temp_list.push(&tempcharacter[(index)..(index + 1)]);
        }


        let mut final_vector = vec![];
        for _index in 0..30 {
            final_vector.push("".to_string());
        }

        for tempvalue in temp_list {
            // let simple_vector = *show_one2(&tempvalue.to_string());
            let simple_vector = self.SimpleWord.get(tempvalue).unwrap();;


            let mut index = 0;
            for x in simple_vector.iter() {
                final_vector[index].push_str(&*x);
                index += 1;
            }
        }

        for temp in final_vector {
            println!("{}", temp)
        }
    }

    pub fn beautifyshow(&self, day: i32, hour: i32, min: i32, sec: i32) {
        // let day = 20;
        // let hour = 1;
        // let min = 1;
        // let sec = 1;
        let datecollect = format!("{:02}:{:02}:{:02}:{:02}", day, hour, min, sec);
        // showtotallabel();
        self.showtotallabel();
        // showtotaldate(&datecollect);
        self.showtotaldate(&datecollect);
    }
}
