#[allow(unused_imports)]
mod inventory_items;
#[allow(unused_imports)]
use crate::inventory_items::inventory_types::ArrayComponent;
#[allow(unused_imports)]
use crate::inventory_items::inventory_types::ArrayComponentType;
#[allow(unused_imports)]
use crate::inventory_items::inventory_types::ArrayConnection;
#[allow(unused_imports)]
use crate::inventory_items::inventory_types::BatteryBank;
#[allow(unused_imports)]
use crate::inventory_items::inventory_types::Display;
#[allow(unused_imports)]
use crate::inventory_items::inventory_types::PanelInventory;
#[allow(unused_imports)]
use crate::inventory_items::inventory_types::SolarPanel;
#[allow(unused_imports)]
use crate::inventory_items::inventory_types::Specification;
#[allow(unused_imports)]
use crate::inventory_items::inventory_types::Unit;
#[allow(unused_imports)]
use crate::inventory_items::inventory_types::ValueType;

#[allow(unused_imports)]
use crate::inventory_items::inventory_types::ArrayConnectionType;

pub struct Inventory {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn spec_factory_test() {
        let mut spec =
            Specification::new(String::from("volts"), ValueType::Float(14.4), Unit::Volts);
        assert_eq!(spec.name, "volts");
        assert_eq!(spec.unit, Unit::Volts);
        spec.unit = Unit::Amps;
        assert_eq!(spec.unit, Unit::Amps);
        spec.unit = Unit::Watts;
        assert_eq!(spec.unit, Unit::Watts);
        //        assert_eq!(spec.value, 24.4);
    }

    #[test]
    fn array_connection_test() {
        let panel = ArrayComponent::new_solar_panel(290, 32.1, 9.05);

        let panel2 = ArrayComponent::new_solar_panel(290, 32.1, 9.05);
        let connection = ArrayConnection::connect(panel, panel2, ArrayConnectionType::Series);
        assert_eq!(connection.total_voltage, 64.2);

        println!("{}", connection.display());
    }
    #[test]
    fn spec_trait_test() {
        let panel = ArrayComponent::new_solar_panel(290, 32.1, 9.05);
        assert_eq!(panel.specs.len(), 3);

        println!("\n");
        //output the specs for this panel
        for spec in panel.specs.iter() {
            println!("{}", spec.display());
        }
        println!("\n");
    }
    #[test]
    fn battery_bank_test() {
        let bat_bank = ArrayComponent::new_battery_bank(String::from("Lifeline AGM"), 12.0, 350);
        assert_eq!(bat_bank.specs.len(), 2);

        println!("\n{}\n", BatteryBank::display(&bat_bank));
    }

    #[test]
    fn item_array_test() {
        let mut solar_array = PanelInventory::new();

        solar_array
            .items
            .push((ArrayComponent::new_solar_panel(290, 32.1, 9.05), 2));
        solar_array
            .items
            .push((ArrayComponent::new_solar_panel(445, 44.46, 10.48), 3));

        assert_eq!(solar_array.items.len(), 2);

        let (panel, _quantity) = &solar_array.items[0];
        assert_eq!(panel.specs.len(), 3);
        println!("{}", solar_array.display());
    }
}
