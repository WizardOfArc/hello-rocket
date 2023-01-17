use clap::Parser;
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;
use std::str::FromStr;


#[derive(clap::ValueEnum, Clone, Debug)]
enum ConversionType {
    ParisToPst,
    PstToParis,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg()]
    time_string: String,

    #[arg(value_enum)]
    conversion_type: ConversionType,

}

#[derive(Debug, Clone)]
struct InvalidTimeString;

impl fmt::Display for InvalidTimeString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid time string")
    }
}

struct HourMinutePair {
    hour: i32,
    minute: i32,
}

fn add_delta(time_a: &HourMinutePair, hour_delta: i32) -> HourMinutePair {
    let summed_hour = match time_a.hour + hour_delta {
        h if h < 0 => h + 24,
        h if h >= 24 => h - 24,
        h => h,
    };

    HourMinutePair{ hour:summed_hour, minute:time_a.minute}
}

fn paris_time_string_to_time(paris_time_string: &String) -> Result<HourMinutePair, InvalidTimeString> {
    lazy_static! {
        static ref RE_PARIS: Regex = Regex::new(r"(?P<hour>\d+)h(?P<minute>\d+)").unwrap();
    }
    let cap = RE_PARIS.captures(paris_time_string).ok_or(InvalidTimeString)?;
    let hour = cap.name("hour").ok_or(InvalidTimeString)?;
    let minute = cap.name("minute").ok_or(InvalidTimeString)?;
    Ok(HourMinutePair{
        hour:<i32 as FromStr>::from_str(hour.as_str()).unwrap(), 
        minute:<i32 as FromStr>::from_str(minute.as_str()).unwrap(),
    })
}

fn time_to_paris_time_string(time: &HourMinutePair) -> String {
    let minute_string = match time.minute {
        m if m < 10 => format!("0{}", m),
        m => format!("{}", m),
    };
    format!("{} h {}", time.hour, minute_string)
}

fn paris_to_pst_string(paris_time_string: &String) -> String {
   let paris_time_result = paris_time_string_to_time(paris_time_string);
   match paris_time_result {
       Ok(paris_time) => {
           let pst_time = add_delta(&paris_time, -9);
           time_to_pst_time_string(&pst_time)
       },
       Err(_) => "InvalidTimeString".to_string()
   }
}

fn pst_to_paris_string(pst_time_string: &String) -> String {
   let pst_time_result = pst_time_string_to_time(pst_time_string);
   match pst_time_result {
       Ok(pst_time) => {
           let paris_time = add_delta(&pst_time, 9);
           time_to_paris_time_string(&paris_time)
       },
       Err(_) => "InvalidTimeString".to_string()
   }
}

fn time_to_pst_time_string(time: &HourMinutePair) -> String {
    match time.hour {
        h if h > 12 => format!("{}:{}PM", h - 12, time.minute),
        12 => format!("12:{}PM", time.minute),
        h => format!("{}:{}AM", h, time.minute),
    }
}

fn pst_time_string_to_time(pst_time_string: &String) -> Result<HourMinutePair, InvalidTimeString> {
    // split string into hours, colon, minutes, am-pm
    lazy_static! {
        static ref RE_PST: Regex = Regex::new(r"(?P<hour>\d+):(?P<minute>\d{2})(?P<meridian>[aApP][mM])").unwrap();
    }
    let cap = RE_PST.captures(pst_time_string).ok_or(InvalidTimeString)?;
    let hour = cap.name("hour").ok_or(InvalidTimeString)?;
    let minute = cap.name("minute").ok_or(InvalidTimeString)?;
    let meridian = cap.name("meridian").ok_or(InvalidTimeString)?;
    let raw_12_hour = <i32 as FromStr>::from_str(hour.as_str()).unwrap();
    let raw_minute = <i32 as FromStr>::from_str(minute.as_str()).unwrap();
    match meridian.as_str() {
        "am" | "Am" | "aM" | "AM" => match raw_12_hour {
            12 => Ok(HourMinutePair{hour:0, minute:raw_minute}),
            h => Ok(HourMinutePair{hour:h, minute:raw_minute}),
        },
        
        "pm" | "Pm" | "pM" | "PM" => match raw_12_hour {
            12 => Ok(HourMinutePair{hour:12, minute:raw_minute}),
            h => Ok(HourMinutePair{hour:12+h, minute:raw_minute}),
        },
        _ => Err(InvalidTimeString),
    }
}

fn main() {
    let args = Args::parse();

    println!("Bonjour!");
    match args.conversion_type {
        ConversionType::ParisToPst => {
            println!("Pacific Standard Time is: {}",paris_to_pst_string(&args.time_string));
        },
        ConversionType::PstToParis => {
            println!("Parisian Time is: {}",pst_to_paris_string(&args.time_string));
        },
    }
}
