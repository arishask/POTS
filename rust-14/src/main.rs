#![allow(dead_code)]
pub struct User {
    name: String,
    age: u32,
    height: f32,
    visit_count: usize,
    // Опциональное поле
    last_blood_pressure: Option<(u32, u32)>,
}

pub struct Measurements {
    height: f32,
    blood_pressure: (u32, u32),
}

// 'a - это время жизни, мы поговорим об этом в следующем разделе
pub struct HealthReport<'a> {
    patient_name: &'a str,
    visit_count: u32,
    height_change: f32,
    // Опциональное поле
    blood_pressure_change: Option<(i32, i32)>,
}

impl User {
    pub fn new(name: String, age: u32, height: f32) -> Self {
        Self {
            name,
            age,
            height,
            visit_count: 0,
            last_blood_pressure: None,
        }
    }

    pub fn visit_doctor(&mut self, measurements: Measurements) -> HealthReport<'_> {
        // Увеличиваем количество посещений врача
        self.visit_count += 1;
        // Показатели кровяного давления из измерений
        let bp = measurements.blood_pressure;
        // Отчет
        let report = HealthReport {
            patient_name: &self.name,
            visit_count: self.visit_count as u32,
            // Изменение роста
            height_change: measurements.height - self.height,
            // Изменение давления.
            // Последнее измерение давления может быть пустым,
            // поэтому выполняется сопоставление с шаблоном
            blood_pressure_change: match self.last_blood_pressure {
                Some(lbp) => {
                    Some((bp.0 as i32 - lbp.0 as i32, bp.1 as i32 - lbp.1 as i32))
                }
                None => None,
            },
        };
        self.height = measurements.height;
        self.last_blood_pressure = Some(bp);
        report
    }
}

fn main() {
    let bob = User::new(String::from("Bob"), 32, 155.2);
    println!("I'm {} and my age is {}", bob.name, bob.age);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visit() {
        let mut bob = User::new(String::from("Bob"), 32, 155.2);
        assert_eq!(bob.visit_count, 0);
        let report = bob.visit_doctor(Measurements {
            height: 156.1,
            blood_pressure: (120, 80),
        });
        assert_eq!(report.patient_name, "Bob");
        assert_eq!(report.visit_count, 1);
        assert_eq!(report.blood_pressure_change, None);

        let report = bob.visit_doctor(Measurements {
            height: 156.1,
            blood_pressure: (115, 76),
        });

        assert_eq!(report.visit_count, 2);
        assert_eq!(report.blood_pressure_change, Some((-5, -4)));
    }
}