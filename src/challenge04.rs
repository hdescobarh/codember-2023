/*
** Hackers dañan sistema de archivos **
En un mundo donde la información es poder, un hacker conocido como Savipo Yatar descubre una serie de archivos en un servidor altamente protegido.

Estos archivos contienen secretos que podrían cambiar el curso de la historia. Pero hay un problema: algunos de los archivos son falsos, diseñados para engañar a los intrusos. Savipo Yatar debe determinar rápidamente cuáles archivos son reales y cuáles son falsos.

Cada archivo tiene un nombre con dos partes, separadas por un guion (-). La primera parte es una cadena alfanumérica y la segunda es un checksum, que es una cadena formada por los caracteres que sólo aparecen una vez en la primera parte y en el orden en que aparecen.

Escribe un programa que determine si un archivo es real o falso basado en estas reglas.

** Cómo resolverlo **
1. Analiza la lista de nombres de archivos y sus checksums que encontrarás en este archivo: https://codember.dev/data/files_quarantine.txt

2. Busca el archivo real número 33 (de todos los archivos reales, el 33º en orden de aparición) y envía su checksum con submit. Por ejemplo si el archivo es xyzz33-xy, harías:
*/

use std::collections::HashMap;

pub fn validate_files(files_quarantine: &str) -> Vec<&str> {
    let mut real_files: Vec<&str> = Vec::new();
    for line in files_quarantine.lines() {
        if let Some(checksum) = parse_line(line) {
            real_files.push(checksum)
        }
    }
    real_files
}

// Returns the checksum if it is valid, returns None otherwise
fn parse_line(line: &str) -> Option<&str> {
    let name_checksum: Vec<&str> = line.split('-').collect();
    if name_checksum.len() != 2 {
        panic!("Bad line formating.")
    }
    let file_name = name_checksum[0];
    let file_checksum = name_checksum[1];
    let mut chars_order: Vec<char> = Vec::new();
    let mut chars_count: HashMap<char, usize> = HashMap::new();

    // counts how many times appears each character and preserve its order
    for c in file_name.chars() {
        let added_value = chars_count
            .entry(c)
            .and_modify(|count| *count += 1)
            .or_insert(1);
        if *added_value == 1 {
            chars_order.push(c)
        }
    }

    let mut calculated_checksum = String::new();

    // construct the correct checksum
    for c in chars_order {
        if chars_count[&c] != 1 {
            continue;
        }
        calculated_checksum.push(c)
    }

    // compare both checksums
    if file_checksum == calculated_checksum {
        return Some(file_checksum);
    }
    None
}

#[cfg(test)]
mod test {
    /*
        Ejemplos:

    Nombre del archivo: xyzz33-xy
    Resultado: ✅ Real (El checksum es válido)

    Nombre del archivo: abcca1-ab1
    Resultado: ❌ Falso (El checksum debería ser b1, es incorrecto)

    Nombre del archivo: abbc11-ca
    Resultado: ❌ Falso (El checksum debería ser ac, el orden es incorrecto)
    Cada línea indica el nombre del archivo y su correspondiente checksum, separados por un guion (-).
     */

    use crate::challenge04::parse_line;

    fn test_examples() {
        let test_cases = [
            (Some("xy"), "xyzz33-xy"),
            (None, "bcca1-ab1"),
            (None, "abbc11-ca"),
        ];

        for (result, case) in test_cases {
            assert_eq!(result, parse_line(case))
        }
    }
}
