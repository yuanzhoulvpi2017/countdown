use image::{open, imageops};
use std::collections::HashMap;
use std::env::current_dir;

#[warn(unused_assignments)]
pub fn show_one2(number: &String) -> Box<Vec<String>> {
    let nwidth = 20;
    let nheight = 30;
    let gap_value = 120 as u8;
    let cur_dir = current_dir().unwrap().into_os_string().into_string().unwrap();

    let imagepath = if number == ":" {
        let temp_number = "maohao";
        let val = format!("{}/NumberSimple/{}.png", &cur_dir, temp_number);
        val
    } else {
        let val = format!("{}/NumberSimple/{}.png", &cur_dir, number);
        val
    };


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

#[warn(unused_assignments)]
pub fn show_label(number: &String) -> Box<Vec<String>> {
    let nwidth = 28 * 2;
    let nheight = 28;
    let gap_value = 120 as u8;
    let cur_dir = current_dir().unwrap().into_os_string().into_string().unwrap();

    let imagepath = format!("{}/NumberSimple/{}.png", &cur_dir, number);

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


pub struct CountDown {
    simple_word: HashMap<&'static str, Box<Vec<String>>>,
}

impl CountDown {
    pub fn new() -> Self {
        let mut dict_result = HashMap::new();
        let biglist = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", ":", "day", "hou", "min", "sec"];
        let number = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", ":"];
        for temp_list in biglist {
            if number.contains(&temp_list) {
                let result = show_one2(&temp_list.to_string());
                dict_result.insert(temp_list, result);
            } else {
                let result = show_label(&temp_list.to_string());
                dict_result.insert(temp_list, result);
            }
        }

        return Self {
            simple_word: dict_result,
        };
    }
    pub fn show_total_label(&self) {
        let temp_list = vec!["day", "hou", "min", "sec"];


        let mut final_vector = vec![];
        for _index in 0..28 {
            final_vector.push("".to_string());
        }

        for tempvalue in temp_list {
            // let simple_vector = *show_label(&tempvalue.to_string());
            let simple_vector = self.simple_word.get(tempvalue).unwrap();


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
            let simple_vector = self.simple_word.get(tempvalue).unwrap();


            let mut index = 0;
            for x in simple_vector.iter() {
                final_vector[index].push_str(&*x);
                // final_vector[index].push_str("****");
                index += 1;
            }
        }

        for temp in final_vector {
            println!("{}", format!("{}****", temp));
        }
    }

    pub fn beautifyshow(&self, day: i32, hour: i32, min: i32, sec: i32) {
        let datecollect = format!("{:02}:{:02}:{:02}:{:02}", day, hour, min, sec);
        self.show_total_label();
        self.showtotaldate(&datecollect);
    }
}
