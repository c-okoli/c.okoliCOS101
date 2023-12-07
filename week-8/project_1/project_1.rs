// Rust Program For Validating The Level Of The Staff In An Organization 

fn main() {
    println!("Staff Validation System");
    
    let mut user = String::new();
    println!("\nEnter The Amount Of Users Using The System");
    std::io::stdin().read_line(&mut user).expect("Failed Input");
    let amount_of_user:u64 = user.trim().parse().expect("Failed Input");

 for _x in 1..=amount_of_user{
      let _v = vec!["Office adminstrator","Academic","Lawyer","Teacher"];
      
      let mut option = String::new();
      
      println!("\nChoose Out Of The Following Options: 0 - Office Administrator, 1 - Academic, 2 - Lawyer,3 - Teacher");
      std::io::stdin().read_line(&mut option).expect("Failed Input");
      let index:usize = option.trim().parse().expect("Invalid Input");
      

    if index == 0{
        
        let _e = vec!["Intern","Administrator","Senior Administrator","Office Manager","Director","CEO"];

        let mut option2 = String::new();
        println!("\nPick Out Of The Following Options Available: 0 - Intern, 1 - Administrator, 2 - Senior Administrator, 3 - Office Manager, 4 - Director,5 - CEO ");
        std::io::stdin().read_line(&mut option2).expect("Failed Input");
        let index2:usize = option2.trim().parse().expect("Failed Input");
         
         if index2 == 0{
            println!("Your Position  Intern is APS 1-2 With 1-2 Years Of Experience");
        }
        if index2 == 1{
            println!("Your Position As Administrator is APS 3-5 With 3-5 Years Of Experience");
        }
        if index2 == 2{
            println!("Your Position As Senior Adminstrator is APS 5-8 With 5-8 Years Of Experience");
        }
        if index2 == 3{
            println!("Your Position As Office Manager is EL1 8-10 With 8-10 Years Of Experience");
        }
        if index2 == 4{
            println!("Your Position As Director is EL2 10-13 With 10-13 Years Of Experience");
        }
        if index2 == 5{
            println!("Your Position As CEO is SES With More Than 13 Years Of Experience");
        }

    }
    if index == 1{

        let _c = vec!["Research Assistant","PhD Candidate","Post-Doc Researcher","Senior Lecturer","Dean"];

        let mut option3 = String::new();
        println!("\nPick Out Of The Following Options Available: 0 - Research Assistant, 1 - PhD Candidate,2 - Post-Doc Researcher, 3 - Senior Lecturer,4 - Dean");
        std::io::stdin().read_line(&mut option3).expect("Failed Input");
        let index3:usize = option3.trim().parse().expect("Failed Input");

          if index3 == 0{
            println!("Your Position As A Research Assistant is APS 3-5 With 3-5 Years Of Experience");
          }
          if index3 == 1{
            println!("Your Position As PhD Candidate is APS 5-8 With 5-8 Years Of Experience");
          }
          if index3 == 2{
            println!("Your Position As Post-Doc Researcher is EL1 8-10 With 8-10 Years Of Experience");
          }
          if index3 == 3{
            println!("Your Position As A Senior Lecturer is EL2 10-13 With 10-13 Years Of Experience");
          }
          if index3 == 4{
            println!("Your Position As A Dean is SES With Over 13 Years Of Experience");
          }
    
   }
   if index == 2{
     
     let _t = vec!["Parallegal","Junior Associate","Associate","Senior Associate 1-2","Senior Associate 3-4","Partner"];
     
     let mut option4 = String::new();
     println!("\nPick Out of The Following Options Available: 0 - Parallegal, 1 - Junior Associate, 2 - Senior Associate 1-2, 3 - Senior Associate 3-4, 4 - Partner");
     std::io::stdin().read_line(&mut option4).expect("Failed Input");
     let index4:usize = option4.trim().parse().expect("Failed Input");

        if index4 == 0{
            println!("Your Position As Parallegal is APS 1-2 With 1-2 Years Of Experience");
        }
        if index4 == 1{
            println!("Your Position As Junior Associate is APS 3-4 With 3-4 Years Of Experience");
        }
        if index4 == 2{
            println!("Your Position As Associate is APS 5-8 With 5-8 Years Of Experience");
        }
        if index4 == 3{
            println!("Your Position As  Senior Associate 1-2 is EL1 8-10 With 8-10 Years Of Experience");
        }
        if index4 == 4{
            println!("Your Position As Senior Associate 3-4 is EL2 10-13 With 10-13 Years Of Experience");
        }
        if index4 == 5{
            println!("Your Position As Dean is SES With Over 13 Years Of Experience");
        }
    }
    if index == 3{

        let _o = vec!["Placement","Classroom Teacher","Snr Teacher","Leading Teacher","Deputy Principal","Principal"];

        let mut option5 = String::new();
        println!("\nPick Out From The Following Options Available: 0 - Placement, 1 - Classroom Teacher, 2 - Snr Teacher, 3 - Leading Teacher, 4 - Deputy Principal, 5 - Principal");
        std::io::stdin().read_line(&mut option5).expect("Failed Input");
        let index5:usize = option5.trim().parse().expect("Failed Input");

           if index5 == 0{
            println!("Your Postion As Placement is APS 1-2 With 1-2 Years Of Experience");
           }
           if index5 == 1{
            println!("Your Position As Classroom Teacher  is APS 2-3 With 3-4 Years Of Experience ");
           }
           if index5 == 2{
            println!("Your Position As Snr Teacher is APS 5-8 With 5-8 Years Of Experience ");
           }
           if index5 == 3{
            println!("Your Position As Leading Teacher is EL1 8-10 With 8-10 Years Of Experience");
           }
           if index5 == 4{
            println!("Your Position As Deputy Principal is EL2 10-13 With 10-13 Years Of Experience");
           }
           if index5 == 5{
            println!("Your Position As Principal is SES With Above 13 Years Of Experience ");
           }
    }

    
      
  }

}

