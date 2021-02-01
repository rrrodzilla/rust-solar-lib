use std::iter::Iterator;

pub mod inventory_types {

    #[derive(Debug, PartialEq)]
    pub enum Unit {
        Amps,
        Volts,
        Watts,
    }

    #[derive(Debug, PartialEq)]
    pub enum ValueType {
        Int(i32),
        Float(f32),
    }
    pub trait Display {
        fn display(&self) -> String;
    }

    #[derive(Debug)]
    pub struct Specification {
        pub name: String,
        pub value: ValueType,
        pub unit: Unit,
    }

    impl Specification {
        pub fn new(name: String, value: ValueType, unit: Unit) -> Specification {
            Specification {
                name: name,
                value: value,
                unit: unit,
            }
        }
    }
    impl Display for Specification {
        fn display(&self) -> String {
            let v: String = match self.value {
                ValueType::Int(value) => value.to_string(),
                ValueType::Float(value) => value.to_string(),
            };

            String::from(format!("{} {:?} - {}", v, self.unit, self.name))
        }
    }

    pub struct PanelInventory {
        pub items: Vec<(ArrayComponent, i32)>,
    }

    #[allow(dead_code)]
    impl PanelInventory {
        pub fn new() -> PanelInventory {
            PanelInventory { items: vec![] }
        }
    }
    impl Display for PanelInventory {
        fn display(&self) -> String {
            let mut display_string = String::from("\nPanel Inventory\n");
            let mut sum = 0;
            for (panel, quantity) in self.items.iter() {
                display_string
                    .push_str(format!("{} - {}", panel.specs[0].display(), quantity).as_str());
                display_string.push_str("\n");
                sum += quantity;
            }
            display_string.push_str("_______________\n");
            display_string.push_str(format!("Total panels: {}\n", sum).as_str());
            display_string
        }
    }

    #[allow(dead_code)]
    pub enum ArrayComponentType {
        SolarPanel,
        SeriesConnection,
        ParallelConnection,
        DirectConnection,
        BatteryBank,
    }

    pub trait SolarPanel {
        fn new_solar_panel(pmax: i32, vmp: f32, imp: f32) -> ArrayComponent;
    }

    pub trait BatteryBank {
        fn new_battery_bank(voltage: f32, total_amp_hours: i32) -> ArrayComponent;
    }

    pub struct ArrayComponent {
        pub name: String,
        pub specs: Vec<Specification>,
        pub component_type: ArrayComponentType,
    }

    impl Display for ArrayComponent {
        fn display(&self) -> String {
            String::from(format!("{}", self.name))
        }
    }
    #[allow(dead_code)]
    impl SolarPanel for ArrayComponent {
        fn new_solar_panel(pmax: i32, vmp: f32, imp: f32) -> ArrayComponent {
            ArrayComponent {
                name: String::from("Solar Panel"),
                component_type: ArrayComponentType::SolarPanel,
                specs: vec![
                    Specification::new(
                        String::from("Nominal Max Power (Pmax)"),
                        ValueType::Int(pmax),
                        Unit::Watts,
                    ),
                    Specification::new(
                        String::from("Opt Operating Voltage (Vmp)"),
                        ValueType::Float(vmp),
                        Unit::Volts,
                    ),
                    Specification::new(
                        String::from("Opt Operating Current (Imp)"),
                        ValueType::Float(imp),
                        Unit::Amps,
                    ),
                ],
            }
        }
    }
}
