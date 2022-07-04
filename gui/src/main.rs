use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "client", about = "arguments for client")]
#[structopt(settings = &[structopt::clap::AppSettings::AllowLeadingHyphen])]
struct Opt {
    ///Testing
    val1: String,

    ///Debugging
    val2: String,
}

fn main() {
    let opt = Opt::from_args();
    let equ = Equation::new(
        opt.val1.parse::<i64>().unwrap() as u32,
        opt.op,
        opt.val2.parse::<i64>().unwrap() as u32,
    );
    printsol(equ);
}
