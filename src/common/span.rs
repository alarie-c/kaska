use std::ops::Range;
pub type Span = Range<usize>;

pub fn line_number(span: &Span, source: &String) -> usize {
    return source[..span.start].chars().filter(|&c| c == '\n').count() + 1;
}

pub fn formatted_content(span: &Span, underline: &Span, source: &String) -> Option<String> {
    if span.end > source.len() || span.start > span.end {
        return None;
    }

    let line_start = source[0..span.start]
        .rfind('\n')
        .map_or(0, |n| span.start - n);

    let line_end = source[span.end..source.len()]
        .find('\n')
        .map_or(source.len(), |n| span.end + 1 + n);

    let mut text: Vec<String> = vec![String::new()];
    let mut carets: Vec<String> = vec![String::new()];
    
    let mut i = line_start;
    let mut j = 0;
    'builder: loop {
        if i >= line_end {
            break 'builder;
        }
        
        let c = source.as_bytes()[i] as char;
        
        if c == '\n' {
            j += 1;
            continue 'builder;
        }
        
        if underline.contains(&i) {
            carets[j].push('^');
        } else {
            carets[j].push(' ');
        }

        text[j].push(c);
        i += 1;
    }

    let mut result = String::new();
    let i = 0;
    while i < text.len() {
        result.push_str(format!("{}\n{}\n", text[i], carets[i]).as_str());
    }
    return Some(result);
}
