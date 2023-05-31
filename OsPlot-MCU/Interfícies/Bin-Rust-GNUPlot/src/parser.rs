use std::str::Split;

pub fn ordre_os(ordres: &mut Split<char>) -> Option<u8> {
    let mut factor_oversampling = ordres.next();
    while factor_oversampling == Some("") { factor_oversampling = ordres.next(); }
    if factor_oversampling.is_none() { println!("No s'ha donat el factor d'oversampling"); return None }
    let factor_oversampling =
        u8::from_str_radix(factor_oversampling.unwrap(), 10);
    if factor_oversampling.is_err() { println!("El factor d'oversampling donat no és un número vàl·lid"); return None }
    let factor_oversampling = factor_oversampling.unwrap();
    if factor_oversampling > 20 { println!("El factor d'oversampling no pot ser major de 20"); return None }
    else if factor_oversampling == 0 { println!("El factor d'oversampling no pot ser 0"); return None }
    return Some(factor_oversampling);
}

pub fn ordre_n(ordres: &mut Split<char>) -> Option<u16> {
    let mut n_mostres = ordres.next();
    while n_mostres == Some("") { n_mostres = ordres.next(); }
    if n_mostres.is_none() { println!("No s'ha donat el nombre de mostres"); return None }
    let n_mostres =
        u16::from_str_radix(n_mostres.unwrap(), 10);
    if n_mostres.is_err() { println!("El nombre de mostres donat no és un número vàl·lid"); return None }
    let n_mostres = n_mostres.unwrap();
    if n_mostres > 1000 { println!("El nombre de mostres no pot ser major de 1000"); return None }
    else if n_mostres == 0 { println!("El nombre de mostres no pot ser 0"); return None }
    return Some(n_mostres);
}

pub fn ordre_tr(ordres: &mut Split<char>) -> Option<u8> {
    let mut nivell_trigger = ordres.next();
    while nivell_trigger == Some("") { nivell_trigger = ordres.next(); }
    if nivell_trigger.is_none() { println!("No s'ha donat el nivell de trigger"); return None }
    let nivell_trigger = nivell_trigger.unwrap().parse::<f32>();
    if nivell_trigger.is_err() { println!("El nivell de trigger donat no és un número vàl·lid"); return None }
    let nivell_trigger = nivell_trigger.unwrap();
    if nivell_trigger > 5. { println!("El nivell de trigger no pot ser major de 5"); return None }
    else if nivell_trigger < 0. { println!("El nivell de trigger no pot ser menor que 0"); return None }
    return Some((nivell_trigger*255./5.) as u8);
}