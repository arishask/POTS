use std::io::{Read, Result};

struct RotDecoder<R: Read> {
    input: R,
    rot: u8,
}

impl<R: Read> Read for RotDecoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        // Читаем данные в буфер
        let size = self.input.read(buf)?;
        // Перебираем байты
        for b in &mut buf[..size] {
            // Только буквы алфавита
            if b.is_ascii_alphabetic() {
                // База
                let base = if b.is_ascii_uppercase() { 'A' } else { 'a' } as u8;
                // Сдвигаем на `rot` в пределах 26 (количество букв в английском алфавите)
                *b = (*b - base + self.rot) % 26 + base;
            }
        }
        // Возвращаем "сдвинутые" байты
        Ok(size)
    }
}

fn main() {
    let mut rot =
        RotDecoder { input: "Gb trg gb gur bgure fvqr!".as_bytes(), rot: 13 };

    let mut result = String::new();
    rot.read_to_string(&mut result).unwrap();

    println!("{}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn joke() {
        let mut rot =
            RotDecoder { input: "Gb trg gb gur bgure fvqr!".as_bytes(), rot: 13 };
        let mut result = String::new();
        rot.read_to_string(&mut result).unwrap();
        assert_eq!(&result, "To get to the other side!");
    }

    #[test]
    fn binary() {
        let input: Vec<u8> = (0..=255u8).collect();
        let mut rot = RotDecoder::<&[u8]> { input: input.as_ref(), rot: 13 };
        let mut buf = [0u8; 256];
        assert_eq!(rot.read(&mut buf).unwrap(), 256);

        for i in 0..=255 {
            if input[i] != buf[i] {
                assert!(input[i].is_ascii_alphabetic());
                assert!(buf[i].is_ascii_alphabetic());
            }
        }
    }
}