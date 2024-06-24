use pest::iterators::Pair;
use pest_derive::Parser;

#[derive(Parser)]
// Let the record show this is some insanely smart parsing. However, it's kind of restrictive and results in wrong line numbers in some cases.
#[grammar = "asm_source.pest"]
pub struct MipsParser;

#[derive(Debug, Clone)]
pub enum MipsCST<'a> {
    Label(&'a str),
    Instruction(&'a str, Vec<&'a str>),
    Sequence(Vec<MipsCST<'a>>),
    Directive(&'a str),
    Keyword(&'a str),
}

// The following method unpacks the MipsCST instruction into a set of owned values. It's used in assembly.
impl<'a> MipsCST<'a> {
    pub fn unpack_instruction(&self) -> Option<(String, Vec<String>)> {
        match self {
            MipsCST::Instruction(mnemonic, args) => {
                Some(((*mnemonic).to_string(), args.iter().map(|&arg| arg.to_string()).collect()))
            },
            _ => None,
        }
    }
}

pub fn parse_rule(pair: Pair<Rule>) -> MipsCST {
    match pair.as_rule() {
        Rule::vernacular => MipsCST::Sequence(pair.into_inner().map(parse_rule).collect()),
        Rule::label => MipsCST::Label(pair.into_inner().next().unwrap().as_str()),
        Rule::instruction => {
            let mut inner = pair.into_inner();
            let opcode = inner.next().unwrap().as_str();
            let args = inner.clone().map(|p| p.as_str()).collect::<Vec<&str>>();
            MipsCST::Instruction(opcode, args)
        }
        Rule::directive => MipsCST::Directive(pair.into_inner().next().unwrap().as_str()),
        Rule::keyword => MipsCST::Keyword(pair.into_inner().next().unwrap().as_str()),
        _ => {
            println!("Unreachable: {:?}", pair.as_rule());
            unreachable!()
        }
    }
}

pub fn cst_map(cst: &MipsCST, f: fn(&MipsCST) -> ()) {
    match cst {
        MipsCST::Sequence(v) => {
            let _ = v.iter().map(f);
        }
        _ => f(cst),
    }
}

pub fn print_cst(cst: &MipsCST) {
    match cst {
        MipsCST::Label(s) => println!("{}:", s),
        MipsCST::Instruction(mnemonic, args) => println!("\t{} {}", mnemonic, args.join(", ")),
        MipsCST::Sequence(v) => {
            for sub_cst in v {
                print_cst(sub_cst)
            }
        }
        MipsCST::Directive(s) => println!("Directive found: {}", s),
        MipsCST::Keyword(s) => println!("Keyword encountered: {}", s),
    }
}

pub fn instr_to_str(mnemonic: &str, args: &[&str]) -> String {
    format!("{} {}", mnemonic, args.join(" "))
}
