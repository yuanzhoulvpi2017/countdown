use std::io::Read;
use crate::gen::{show_label, show_one2, CountDown};
use crossterm::terminal::{Clear, ClearType};

mod gen;

use chrono;
use chrono::{TimeZone, Utc};

use std::{thread, time};
macro_rules! numin {
      () =>{
          {
            let mut input = String::new();
              std::io::stdin()
                  .read_line(&mut input)
                .expect("Failed to read line");
              input.trim().parse().unwrap()
        }
    };
}


fn main() {
    let countdown = CountDown::new();

    // let mut imonth = 0;
    // let mut iday = 0;
    // let mut ihour = 0;
    // let mut imin = 0;
    // let mut isec = 0;


    println!("input year: ");
    let iyear = numin!();

    println!("input month: ");
    let imonth = numin!();

    println!("input day: ");
    let iday = numin!();

    println!("input hour: ");
    let ihour = numin!();

    let delay = time::Duration::from_secs(1);

    loop {
        let curdate = Utc::now().timestamp();
        let targetdate = Utc.ymd(iyear, imonth, iday).and_hms(ihour, 0, 0);

        let minus_timestamp = targetdate.timestamp() - curdate;
        // let minus_timestamp = targetdate2.timestamp() - curdate;


        let days = minus_timestamp / (86400);
        let hours = (minus_timestamp % (86400)) / (3600);
        let mins = (minus_timestamp - days * 86400 - hours * 3600) / 60;
        let secs = (minus_timestamp - days * 86400 - hours * 3600) % 60;


        countdown.beautifyshow(days as i32, hours as i32, mins as i32, secs as i32);

        thread::sleep(delay);
        Clear(ClearType::All);
        // std::process::Command::new("cls").status().unwrap();
    }
}

