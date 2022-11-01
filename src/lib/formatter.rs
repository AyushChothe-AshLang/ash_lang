use super::nodes::Node;

pub struct Formatter {
    indent: u8,
    times: u8,
}

impl Formatter {
    pub fn new(indent: u8) -> Self {
        Formatter { indent, times: 0 }
    }

    fn space(&self) -> String {
        let mut _out = "".to_string();
        for _ in 1..self.times {
            let mut _s = "".to_string();
            for _ in 0..self.indent {
                _s += " ";
            }
            _out += _s.as_str();
        }
        _out
    }

    pub fn format(&mut self, node: Node) -> String {
        match node {
            Node::Int(_i) => format!("{}", _i.value),
            Node::Double(_d) => format!("{}", _d.value),
            Node::Boolean(_b) => format!("{}", _b.value),
            Node::String(_s) => format!("\"{}\"", _s.value),
            Node::Identifier(_id) => format!("{}", _id.value),
            Node::UnaryNumber(_un) => format!("{}{}", _un.op, _un.value),
            Node::UnaryBoolean(_ub) => format!("{}{}", _ub.op, _ub.value),
            Node::BinaryOpNumber(_bon) => format!("{} {} {}", _bon.left, _bon.op, _bon.right),
            Node::BinaryOpBoolean(_bob) => format!("{} {} {}", _bob.left, _bob.op, _bob.right),
            Node::Assignment(_a) => format!("{} {} {}", _a.id, _a.assign_type, _a.value),
            Node::Declaration(_dec) => format!("let {} = {};", _dec.id, _dec.value),
            Node::MultiDeclaration(_mdec) => {
                format!("let ")
                    + format!(
                        "{}",
                        _mdec
                            .declarations
                            .iter()
                            .map(|_d| {
                                if let Node::Declaration(_dec) = _d {
                                    return format!("{} = {}", _dec.id, _dec.value);
                                }
                                return format!("");
                            })
                            .collect::<Vec<String>>()
                            .join(", ")
                    )
                    .as_str()
                    + format!(";").as_str()
            }
            Node::BlockStatement(_blk) => {
                let mut out = if self.times != 0 {
                    String::from("{")
                } else {
                    String::from("")
                };
                self.times += 1;
                for _n in &_blk.value {
                    out += "\n";
                    out += self.space().as_str();
                    if let &Node::FunctionCall(_) = _n {
                        out += format!("{};\n", _n).as_str();
                    } else {
                        out += format!("{}\n", self.format(_n.clone())).as_str();
                    }
                }
                self.times -= 1;
                if self.times != 0 {
                    out += self.space().as_str();
                    out += "}";
                }
                out
            }
            Node::FunctionDeclaration(_fnd) => {
                format!("fn {}() {}", _fnd.id, self.format(*_fnd.body))
            }
            Node::FunctionCall(_fnc) => {
                format!("{}(", _fnc.id);
                format!(
                    "{}",
                    _fnc.args
                        .iter()
                        .map(|_a| {
                            return format!("{}", _a);
                        })
                        .collect::<Vec<String>>()
                        .join(", ")
                );
                format!(")")
            }

            Node::Return(_rtn) => {
                if let Some(res) = &_rtn.res {
                    format!("return {};", res)
                } else {
                    format!("return;")
                }
            }
            Node::Break => format!("break;"),
            Node::Continue => format!("continue;"),
            Node::List(_l) => {
                format!("[");
                format!(
                    "{}",
                    _l.elements
                        .iter()
                        .map(|_e| {
                            return format!("{}", _e);
                        })
                        .collect::<Vec<String>>()
                        .join(", ")
                );
                format!("]")
            }
            Node::Map(_m) => {
                format!("{{");
                format!(
                    "{}",
                    _m.elements
                        .iter()
                        .map(|_e| {
                            return format!("{}:{}", _e.0, _e.1);
                        })
                        .collect::<Vec<String>>()
                        .join(", ")
                );
                format!("}}")
            }
            Node::WhileLoop(_w) => format!("while ({}) {}", _w.condition, self.format(*_w.body)),
            Node::IfStatement(_if) => {
                format!("if ({}) {}", _if.condition, self.format(*_if.true_block))
                    + format!(
                        "{}",
                        _if.elif_blocks
                            .iter()
                            .map(|_e| {
                                return self.format(_e.clone());
                            })
                            .collect::<Vec<String>>()
                            .join("")
                    )
                    .as_str()
                    + if _if.else_block.is_some() {
                        format!(" else {}", self.format(*_if.else_block.unwrap()))
                    } else {
                        format!("")
                    }
                    .as_str()
            }
            Node::ElifStatement(_elif) => format!(
                " elif ({}) {}",
                _elif.condition,
                self.format(*_elif.true_block)
            ),
        }
        .to_string()
    }
}
