fn trim(ip) {
  let i = 0;
  let s = -1, e = -1, out = "";
  while (i < len(ip)) {
    let c = get(ip, i);
    if ((s == -1) & (c != " ")) {
      s = i;
      out += c;
    } elif (c != " ") {
      e = i;
      out += c;
    }
    i += 1;
  }
  return out;
}

fn split(ip, sep) {
  let i = 0, groups = [];
  let acc = "";
  while (i < len(ip)) {
    let c = get(ip, i);
    if (c == sep) {
      if (len(trim(acc)) != 0) {
        groups += [acc];
      }
      acc = "";
    } else {
      acc += c;
    }
    i += 1;
  }
  if (len(trim(acc)) != 0) {
    groups += [acc];
  }
  return groups;
}

fn join(ip, sep) {
  let i = 1, out = get(ip, 0);
  while (i < (len(ip) - 1)) {
    out += sep + get(ip, i);
    i += 1;
  }
  out += sep + get(ip, i);
  return out;
}

fn main() {
  println(join(split("  Ayush  Mahesh   Chothe", " "), "_"));
}