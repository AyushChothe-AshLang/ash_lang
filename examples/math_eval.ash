fn lexer(code){
  let digits=["0","1","2","3","4","5","6","7","8","9"];
  let pos = 0;
  let tokens = [];

  while(pos<len(code)){
    let ch = get(code,pos);

    if(has(digits, ch)){
      let d = "";

      while(pos<len(code) & has(digits, ch)){
        d+= ch;
        pos+=1;
        if(pos<len(code)){
          ch = get(code,pos);
        }
      }

      tokens += [{"type": "DIGIT", "value": int(d)}];
    }elif(ch=="+" | ch=="-"){
      tokens += [{"type": "OP", "value": ch}];
      pos+=1;
    }else{
      println("Invalid Token: " + ch);
      pos=len(code);
    }
  }

  return tokens;
}

fn parser(tokens){
  let ast = [];
  let pos = 0;
  
  let left = {"type":"DIGIT", "value": get(get(tokens, pos),"value")};
  pos += 1;

  while(pos<len(tokens) & get(get(tokens, pos),"type")=="OP"){
    let op = get(get(tokens, pos),"value");
    if(op=="+"){
      pos+=1;
      left = {"OP":"Add","left":left,"right":{"type":"DIGIT", "value": get(get(tokens, pos),"value")}};
      if(pos+1<len(tokens)){
        pos+=1;
      }
    }elif(op=="-"){
      pos+=1;
      left = {"OP":"Sub","left":left,"right":{"type":"DIGIT", "value": get(get(tokens, pos),"value")}};
      if(pos+1<len(tokens)){
        pos+=1;
      }
    }
  }

  return left;
}

fn eval(ast){
  if(!has(ast,"OP")){
    return get(ast,"value");
  }

  let cmd = get(ast,"OP");
  let left = get(ast,"left");
  let right = get(ast,"right");

  if(cmd=="Add"){
    return eval(left)+eval(right);
  }elif(cmd=="Sub"){
    return eval(left)-eval(right);
  }
}

fn main(){
  let code = "12+22+4-6";
  let tkns = lexer(code);
  let ast = parser(tkns);
  println(eval(ast));
}