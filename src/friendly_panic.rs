use std::fmt;
use std::panic;
use std::panic::PanicInfo;

#[derive(Debug, Clone)]
pub struct UserPanic {
    pub error_msg: &'static str,
    pub fix_instructions: Option<Vec<Instructions>>,
}
#[derive(Debug, Clone)]
pub struct Instructions {
    pub opener: &'static str,
    pub instructions: Option<Vec<&'static str>>,
}

impl fmt::Display for Instructions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        if let Some(insts) = &self.instructions {
            s = s + &format!("{}\n", self.opener);
            let mut i = 1;
            for inst in insts {
                s = s + &format!("\t{}. {}\n", i, inst);
                i += 1;
            }
        } else {
            s = s + &format!("{}\n", self.opener);
        }

        write!(f, "{}", s)
    }
}

impl fmt::Display for UserPanic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::from("Whoops!\nUnrecoverable error occerred\n\n");
        if self.fix_instructions.is_none() {
            s = s + "It seems like an error that can't be fixed by you!\nPlease follow the following instructions to submit a Bug report to Developer\n";
        } else {
            s = s + "It seems like an error that can be fixed by you!\nPlease follow the following instructions to try and fix the bug\n";
            let insts = self.fix_instructions.as_ref().unwrap();
            let mut i = 1;
            for inst in insts {
                s = s + &format!("{}.  {}\n", i, inst);
                i += 1;
            }
            s = s + "if you are unable to fix errors after following the fix steps.\nPlease Submit a Bug Report to the Developer\n";
        }
        s = s + "Developer contact Steps\n";
        write!(f, "{}", s)
    }
}

pub fn set_hooks() {
    let _org = panic::take_hook();
    panic::set_hook(Box::new(panic_func::<UserPanic>))
}

fn panic_func<Usr: 'static + fmt::Display>(panic_info: &PanicInfo) {
    // need to write a function that writes the trace back to a file.
    // TODO
    println!("{}", panic_info.payload().downcast_ref::<Usr>().unwrap());
}
