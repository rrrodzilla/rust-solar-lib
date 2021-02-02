use std::iter::Iterator;

pub mod inventory_types {

    #[derive(Debug, PartialEq)]
    pub enum Unit {
        Amps,
        Volts,
        Watts,
    }

    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum ValueType {
        Int(i32),
        Float(f32),
    }
    pub trait Display {
        fn display(&self) -> String;
        fn display_specs(&self) -> String {
            String::from("")
        }
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
    pub trait Inventory {
        fn get_items() -> Vec<ArrayComponent>;
    }

    pub struct PanelInventory {
        pub items: Vec<(ArrayComponent, i32)>,
    }

    #[allow(dead_code)]
    pub struct ArrayConnection {
        pub connection_type: ArrayConnectionType,
        pub total_voltage: f32,
        pub total_wattage: i32,
        pub max_amperage: f32,
        pub items: Vec<ArrayComponent>,
    }

    #[allow(dead_code)]
    impl ArrayConnection {
        pub fn add_component(&mut self, to: ArrayComponent) {
            self.items.push(to);
            self.update_totals();
        }
        pub fn connect(
            from: ArrayComponent,
            to: ArrayComponent,
            connection_type: ArrayConnectionType,
        ) -> ArrayConnection {
            //here we should do some error checking
            let mut a = ArrayConnection {
                connection_type: connection_type,
                total_voltage: 0.0,
                total_wattage: 0,
                max_amperage: 0.0,
                items: vec![],
            };

            a.items.push(from);
            a.items.push(to);
            a.update_totals();
            a
        }
        fn update_totals(&mut self) {
            //clear values
            self.total_voltage = 0.0;
            self.total_wattage = 0;
            self.max_amperage = 0.0;
            for item in self.items.iter() {
                // voltage
                let voltage_value =
                    item.get_spec_value(String::from("Opt Operating Voltage (Vmp)"));
                let voltage = match voltage_value {
                    ValueType::Float(v) => v,
                    _ => 0.0,
                };
                match self.connection_type {
                    ArrayConnectionType::Series => (self.total_voltage += voltage),
                    ArrayConnectionType::Parallel => self.total_voltage = voltage,
                    ArrayConnectionType::Direct => self.total_voltage += 0.0,
                };
                // end voltage

                // amperage
                let amperage_value =
                    item.get_spec_value(String::from("Opt Operating Current (Imp)"));
                let amps = match amperage_value {
                    ValueType::Float(a) => a,
                    _ => 0.0,
                };
                match self.connection_type {
                    ArrayConnectionType::Series => self.max_amperage = amps,
                    ArrayConnectionType::Parallel => self.max_amperage += amps,
                    ArrayConnectionType::Direct => self.max_amperage += 0.0,
                };
                // end amperage
                // watts
                let watts_value = item.get_spec_value(String::from("Nominal Max Power (Pmax)"));
                let watts = match watts_value {
                    ValueType::Int(a) => a,
                    _ => 0,
                };
                self.total_wattage += watts;
                //watts
            }
        }
    }
    impl Display for ArrayConnection {
        fn display(&self) -> String {
            let mut s = String::from(
                format!(
                    "Connecting the following items in {:?}:\n",
                    self.connection_type
                )
                .as_str(),
            );
            for item in &self.items {
                s.push_str(Display::display(item).as_str());
            }
            s.push_str(format!("Total Voltage: {}\n", self.total_voltage).as_str());
            s.push_str(format!("Total Amperage: {}\n", self.max_amperage).as_str());
            s.push_str(format!("Total Wattage: {}\n", self.total_wattage).as_str());
            s
        }
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

    pub enum ArrayComponentType {
        SolarPanel,
        BatteryBank,
    }
    #[allow(dead_code)]
    #[derive(Debug)]
    pub enum ArrayConnectionType {
        Series,
        Parallel,
        Direct,
    }

    pub trait SolarPanel {
        fn new_solar_panel(pmax: i32, vmp: f32, imp: f32) -> ArrayComponent;
    }

    pub trait BatteryBank {
        fn new_battery_bank(name: String, voltage: f32, total_amp_hours: i32) -> ArrayComponent;
        fn display(&self) -> String;
    }

    pub struct ArrayComponent {
        pub name: String,
        pub specs: Vec<Specification>,
        pub component_type: ArrayComponentType,
    }

    impl ArrayComponent {
        pub fn get_spec_value(&self, name: String) -> ValueType {
            let mut iter = self.specs.iter();
            let spec = iter.find(|&x| x.name == name);
            match spec {
                Some(v) => v.value,
                None => ValueType::Int(0),
            }
        }
    }

    impl Display for ArrayComponent {
        fn display(&self) -> String {
            let watts = match self.specs[0].value {
                ValueType::Int(a) => a,
                _ => 0,
            };
            String::from(format!(
                "{} - {} {:?}\n",
                self.name, watts, self.specs[0].unit
            ))
        }
    }
    #[allow(dead_code)]
    impl BatteryBank for ArrayComponent {
        fn new_battery_bank(name: String, voltage: f32, total_amp_hours: i32) -> ArrayComponent {
            ArrayComponent {
                name: name,
                component_type: ArrayComponentType::BatteryBank,
                specs: vec![
                    Specification::new(
                        String::from("Voltage"),
                        ValueType::Float(voltage),
                        Unit::Volts,
                    ),
                    Specification::new(
                        String::from("Amp Hours"),
                        ValueType::Int(total_amp_hours),
                        Unit::Volts,
                    ),
                ],
            }
        }
        fn display(&self) -> String {
            let mut s = Display::display(self);
            s.push_str(String::from("\n_______________\n").as_str());
            for spec in &self.specs {
                s.push_str(format!("{}\n", spec.display()).as_str());
            }
            //return display string
            s
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
