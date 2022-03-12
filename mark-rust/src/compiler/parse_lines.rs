use super::block::block;
use super::node::Node;

pub fn parse_lines(code: String) {
    let mut skip_buffer: usize = 0;
    let mut dom: Vec<Node> = Vec::new();
    for (idx, line) in code.lines().enumerate() {
        // skip lines if needed, most probably because of a block
        if skip_buffer > 0 {
            skip_buffer -= 1;
            continue;
        }

        let trimmed = line.trim().to_string();
        // skip empty lines
        if trimmed == "" {
            continue;
        }

        // handle blocks
        if trimmed.contains("`") {
            let (n, s) = block(line, trimmed, code.clone(), idx);
            skip_buffer += s;
            dom.push(n);
        } else if trimmed.starts_with("and ") || trimmed.starts_with("and\n") {
            let n = dom[dom.len() - 1].chain(&trimmed);
            dom.pop();
            dom.push(n);
        } else {
            dom.push(Node::new(line));
        }
    }
    println!(
        "{}",
        Node::new("      div .hi #id .ho .hu attr x='y'").attributes()
    );
}
