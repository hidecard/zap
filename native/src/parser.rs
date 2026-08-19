use crate::value::Param;

pub(crate) fn parse_signature(raw: &str) -> Result<(Vec<Param>, Option<String>), String> {
    let close = raw
        .rfind(')')
        .ok_or("function signature is missing ')'".to_string())?;
    let params_raw = &raw[..close];
    let suffix = raw[close + 1..].trim();
    let return_annotation = suffix
        .strip_prefix("->")
        .map(str::trim)
        .filter(|x| !x.is_empty())
        .map(str::to_string);
    let params = params_raw
        .split(',')
        .filter(|x| !x.trim().is_empty())
        .map(|item| {
            let item = item.trim();
            let (name, annotation) = item
                .split_once(':')
                .map(|(n, a)| (n.trim().to_string(), Some(a.trim().to_string())))
                .unwrap_or((item.to_string(), None));
            if name.is_empty() {
                Err("parameter name cannot be empty".to_string())
            } else {
                Ok(Param { name, annotation })
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok((params, return_annotation))
}

pub(crate) fn static_literal_type(raw: &str) -> Option<&'static str> {
    let value = raw.trim();
    if value.len() >= 2
        && ((value.starts_with('"') && value.ends_with('"'))
            || (value.starts_with('\'') && value.ends_with('\'')))
    {
        Some("text")
    } else if value.parse::<i64>().is_ok() {
        Some("number")
    } else if value == "true" || value == "false" {
        Some("bool")
    } else if value == "none" {
        Some("none")
    } else if value.starts_with('[') && value.ends_with(']') {
        Some("list")
    } else if value.starts_with('{') && value.ends_with('}') {
        Some("map")
    } else {
        None
    }
}
pub(crate) fn split_static_args(raw: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut depth = 0i32;
    let mut quote = None;
    for ch in raw.chars() {
        if let Some(q) = quote {
            current.push(ch);
            if ch == q {
                quote = None;
            }
            continue;
        }
        if ch == '"' || ch == '\'' {
            quote = Some(ch);
            current.push(ch);
        } else if ch == '(' || ch == '[' || ch == '{' {
            depth += 1;
            current.push(ch);
        } else if ch == ')' || ch == ']' || ch == '}' {
            depth -= 1;
            current.push(ch);
        } else if ch == ',' && depth == 0 {
            if !current.trim().is_empty() {
                args.push(current.trim().to_string());
            }
            current.clear();
        } else {
            current.push(ch);
        }
    }
    if !current.trim().is_empty() {
        args.push(current.trim().to_string());
    }
    args
}
pub(crate) fn matching_paren(line: &str, open: usize) -> Option<usize> {
    let chars: Vec<char> = line.chars().collect();
    let mut depth = 0i32;
    let mut quote = None;
    for (index, ch) in chars.iter().enumerate().skip(open) {
        if let Some(q) = quote {
            if *ch == q {
                quote = None;
            }
            continue;
        }
        if *ch == '"' || *ch == '\'' {
            quote = Some(*ch);
        } else if *ch == '(' {
            depth += 1;
        } else if *ch == ')' {
            depth -= 1;
            if depth == 0 {
                return Some(index);
            }
        }
    }
    None
}
pub(crate) fn generic_type(base: &str, inner: &str) -> String {
    format!("{base}<{}>", inner.trim())
}

fn generic_parts(annotation: &str) -> Option<(&str, &str)> {
    let open = annotation.find('<')?;
    if !annotation.ends_with('>') || open == 0 {
        return None;
    }
    Some((
        &annotation[..open],
        &annotation[open + 1..annotation.len() - 1],
    ))
}

fn split_type_args(inner: &str) -> Option<Vec<&str>> {
    let mut args = Vec::new();
    let mut start = 0;
    let mut depth = 0usize;
    for (index, ch) in inner.char_indices() {
        match ch {
            '<' => depth += 1,
            '>' => depth = depth.checked_sub(1)?,
            ',' if depth == 0 => {
                let part = inner[start..index].trim();
                if part.is_empty() {
                    return None;
                }
                args.push(part);
                start = index + 1;
            }
            _ => {}
        }
    }
    if depth != 0 {
        return None;
    }
    let part = inner[start..].trim();
    if part.is_empty() {
        return None;
    }
    args.push(part);
    Some(args)
}

pub(crate) fn is_allowed_annotation(annotation: &str) -> bool {
    let value = annotation.trim();
    if [
        "text", "number", "bool", "list", "map", "object", "none", "any", "result", "option",
    ]
    .contains(&value)
    {
        return true;
    }
    let Some((base, inner)) = generic_parts(value) else {
        return false;
    };
    let Some(args) = split_type_args(inner) else {
        return false;
    };
    match base {
        "list" | "option" | "result" => args.len() == 1 && is_allowed_annotation(args[0]),
        "map" => args.len() == 2 && args.iter().all(|arg| is_allowed_annotation(arg)),
        _ => false,
    }
}

pub(crate) fn annotation_matches(expected: &str, actual: &str) -> bool {
    let expected = expected.trim();
    let actual = actual.trim();
    if expected == "any" || expected == actual {
        return true;
    }
    let (Some((expected_base, expected_inner)), Some((actual_base, actual_inner))) =
        (generic_parts(expected), generic_parts(actual))
    else {
        return false;
    };
    if expected_base != actual_base {
        return false;
    }
    let (Some(expected_args), Some(actual_args)) = (
        split_type_args(expected_inner),
        split_type_args(actual_inner),
    ) else {
        return false;
    };
    expected_args.len() == actual_args.len()
        && expected_args
            .iter()
            .zip(actual_args)
            .all(|(expected, actual)| annotation_matches(expected, actual))
}
