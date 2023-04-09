use std::{ io, time::Instant};


fn main() {

    loop {

        let mut pause = String::from(""); //I am using an empty string here so line 22 has a variable to write to

        let stu_name = get_name();

        if stu_name.trim() == "END" {

            break;

        }

        let time_now = Instant::now();

        println!("Press enter to clock back in");

        io::stdin().read_line(&mut pause).expect("Failed to read line, contact admin");

        let time_elapsed = get_elapsed_time(&time_now);

        println!("{} was out: {}", stu_name.trim(), time_elapsed);

    }


}

fn get_name() -> String {

    let mut stu_name = String::new();

    println!("Enter the student's name, then press enter.");

    io::stdin().read_line(&mut stu_name).expect("Failed to read line, contact admin");

    return stu_name;
}

fn get_elapsed_time(time_now:& Instant) -> String {

    let time_elapsed: u64 = time_now.elapsed().as_secs();

    let minutes = time_elapsed / 60; // Get minutes: 90 = ->1.30

    let seconds = time_elapsed % 60; // Get seconds: 90 = 1.30<-

    for i in 0..9 { // this loop is to add an extra zero to the seconds to fit the MM:SS format

        let i :u64 = i.try_into().expect("Failed to convert 'i' to u64");

        if seconds == i {

            let time_elapsed = format!("{}{}0{}", minutes.to_string(), ":", seconds.to_string());

            return time_elapsed;

        }
    }
    let time_elapsed = format!("{}{}{}", minutes.to_string(), ":", seconds.to_string());

    return time_elapsed;

}
