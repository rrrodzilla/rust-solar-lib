#[allow(unused_imports)]
mod inventory_items;
#[allow(unused_imports)]
use crate::inventory_items::inventory_types::ArrayComponent;
#[allow(unused_imports)]
use crate::inventory_items::inventory_types::ArrayComponentType;
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
